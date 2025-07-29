use dora_node_api::{self, DoraNode, Event, Parameter, dora_core::config::DataId};
use kornia_io::stream::StreamCapture;

fn _get_fraction(fps: f64) -> (u32, u32) {
    if fps.fract() == 0.0 {
        return (fps as u32, 1);
    }

    // commonly used FPS fractions
    let common_fps = [
        (23.976, (24000, 1001)),
        (29.97, (30000, 1001)),
        (59.94, (60000, 1001)),
        (119.88, (120000, 1001)),
    ];
    for &(val, frac) in &common_fps {
        if (fps - val).abs() < 0.001 {
            return frac;
        }
    }

    let mut fps_f = fps;
    let mut denominator = 1;
    while fps_f.fract() != 0.0 {
        fps_f *= 10.0;
        denominator *= 10;
    }

    (fps_f as u32, denominator)
}

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
    let overlay_timestamp = std::env::var("OVERLAY_TIMESTAMP")
        .map(|val| val == "true" || val == "1")
        .unwrap_or(false);

    println!("Configuration:");
    println!("  Size: {}x{}", image_cols, image_rows);
    println!("  FPS: {}", source_fps);
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
    println!("  Overlay timestamp: {}", overlay_timestamp);

    let (fps_numerator, fps_denominator) = _get_fraction(source_fps as f64);

    // create the videotestsrc pipeline
    let pipeline_desc = if overlay_timestamp {
        format!(
            "videotestsrc pattern={} animation-mode={} motion={} background-color={} foreground-color={} flip={} is-live={} ! \
             video/x-raw,width={},height={},framerate={}/{} ! \
             timeoverlay ! \
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
            fps_numerator,
            fps_denominator,
        )
    } else {
        format!(
            "videotestsrc pattern={} animation-mode={} motion={} background-color={} foreground-color={} flip={} is-live={} ! \
             video/x-raw,width={},height={},framerate={}/{} ! \
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
            fps_numerator,
            fps_denominator,
        )
    };

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
