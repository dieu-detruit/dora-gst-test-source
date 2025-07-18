use dora_node_api::{self, DoraNode, Event, Parameter, dora_core::config::DataId};
use kornia_io::stream::StreamCapture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parameters for image size
    let image_cols = std::env::var("IMAGE_COLS")
        .map_err(|e| format!("IMAGE_COLS error: {e}"))?
        .parse::<usize>()?;
    let image_rows = std::env::var("IMAGE_ROWS")
        .map_err(|e| format!("IMAGE_ROWS error: {e}"))?
        .parse::<usize>()?;
    let source_fps = std::env::var("SOURCE_FPS")
        .map_err(|e| format!("SOURCE_FPS error: {e}"))?
        .parse::<u32>()?;

    // parameters for videotestsrc properties
    let animation_mode =
        std::env::var("ANIMATION_MODE").unwrap_or_else(|_| "wall-time".to_string());
    let background_color =
        std::env::var("BACKGROUND_COLOR").unwrap_or_else(|_| "0xff000000".to_string());
    let flip = std::env::var("FLIP").unwrap_or_else(|_| "false".to_string());
    let foreground_color =
        std::env::var("FOREGROUND_COLOR").unwrap_or_else(|_| "0xffffffff".to_string());
    let is_live = std::env::var("IS_LIVE").unwrap_or_else(|_| "true".to_string());
    let motion = std::env::var("MOTION").unwrap_or_else(|_| "0".to_string());
    let pattern = std::env::var("PATTERN").unwrap_or_else(|_| "0".to_string());

    // create the videotestsrc pipeline
    let pipeline_desc = format!(
        "videotestsrc pattern={} animation-mode={} motion={} background-color={} foreground-color={} flip={} is-live={} ! \
         video/x-raw,width={},height={},framerate={}/1 ! \
         videoconvert ! \
         video/x-raw,format=RGB ! \
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
        source_fps
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
                    params.insert("encoding".to_owned(), Parameter::String("RGB8".to_string()));
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
