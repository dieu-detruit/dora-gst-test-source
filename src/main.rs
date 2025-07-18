use dora_node_api::{self, DoraNode, Event, Parameter, dora_core::config::DataId};
use kornia_io::stream::StreamCapture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parameters for image size with default values
    let image_cols = std::env::var("IMAGE_COLS")
        .unwrap_or_else(|_| "640".to_string())
        .parse::<usize>()?;
    let image_rows = std::env::var("IMAGE_ROWS")
        .unwrap_or_else(|_| "480".to_string())
        .parse::<usize>()?;
    let source_fps = std::env::var("SOURCE_FPS")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u32>()?;
    let source_format = std::env::var("SOURCE_FORMAT").unwrap_or_else(|_| "RGB".to_string());

    // parameters for videotestsrc properties with default values
    let animation_mode = std::env::var("ANIMATION_MODE").unwrap_or_else(|_| "0".to_string()); // 0=frames, 1=wall-time, 2=running-time
    let background_color =
        std::env::var("BACKGROUND_COLOR").unwrap_or_else(|_| "0xff000000".to_string());
    let flip = std::env::var("FLIP").unwrap_or_else(|_| "false".to_string());
    let foreground_color =
        std::env::var("FOREGROUND_COLOR").unwrap_or_else(|_| "0xffffffff".to_string());
    let is_live = std::env::var("IS_LIVE").unwrap_or_else(|_| "true".to_string());
    let motion = std::env::var("MOTION").unwrap_or_else(|_| "0".to_string());
    let pattern = std::env::var("PATTERN").unwrap_or_else(|_| "0".to_string()); // 0=smpte, 1=snow, 2=black, etc.

    println!("Configuration:");
    println!("  Size: {}x{}", image_cols, image_rows);
    println!("  FPS: {}", source_fps);
    println!("  Format: {}", source_format);
    println!(
        "  Pattern: {} (0=smpte, 1=snow, 2=black, 3=white, 4=red, 5=green, 6=blue)",
        pattern
    );
    println!(
        "  Animation mode: {} (0=frames, 1=wall-time, 2=running-time)",
        animation_mode
    );
    println!("  Motion: {}", motion);
    println!("  Background color: {}", background_color);
    println!("  Foreground color: {}", foreground_color);
    println!("  Flip: {}", flip);
    println!("  Is live: {}", is_live);

    // create the videotestsrc pipeline
    let pipeline_desc = format!(
        "videotestsrc pattern={} animation-mode={} motion={} background-color={} foreground-color={} flip={} is-live={} ! \
         video/x-raw,width={},height={},framerate={}/1 ! \
         videoconvert ! \
         video/x-raw,format={} ! \
         appsink name=sink",
        pattern,
        animation_mode,
        motion,
        background_color,
        foreground_color,
        flip,
        is_live,
        image_cols,
        image_rows,
        source_fps,
        source_format
    );

    println!("Using GStreamer pipeline: {}", pipeline_desc);

    let mut camera = StreamCapture::new(&pipeline_desc)?;

    // start the camera source
    camera.start()?;

    let output = DataId::from("frame".to_owned());

    let (mut node, mut events) = DoraNode::init_from_env()?;

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id,
                metadata,
                data: _,
            } => match id.as_str() {
                "tick" => {
                    let Some(frame) = camera.grab()? else {
                        continue;
                    };

                    let mut params = metadata.parameters;
                    params.insert(
                        "encoding".to_owned(),
                        Parameter::String(source_format.clone()),
                    );
                    params.insert(
                        "height".to_owned(),
                        Parameter::Integer(frame.size().height as i64),
                    );
                    params.insert(
                        "width".to_owned(),
                        Parameter::Integer(frame.size().width as i64),
                    );

                    // send the frame to the output
                    node.send_output_bytes(
                        output.clone(),
                        params,
                        frame.numel(),
                        frame.as_slice(),
                    )?;
                }
                other => eprintln!("Ignoring unexpected input `{other}`"),
            },
            Event::Stop(_) => {
                camera.close()?;
            }
            other => eprintln!("Received unexpected input: {other:?}"),
        }
    }

    Ok(())
}
