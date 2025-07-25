# dora-gst-test-source

A dora-rs node that generates dummy video using videotestsrc. It uses GStreamer's videotestsrc element to generate various test pattern videos.

## Features

- Dynamic test video generation using GStreamer's videotestsrc
- Generation of various patterns (SMPTE, noise, solid colors, etc.)
- Detailed configuration of size, frame rate, colors, etc.
- Efficient video processing using kornia-io
- Real-time video streaming through integration with dora-rs

## Environment Variables and Default Values

All environment variables have default values, so you can run without configuration.

### Video Settings

| Environment Variable | Default Value | Description |
|---------|-------------|------|
| `IMAGE_COLS` | `640` | Video width (pixels) |
| `IMAGE_ROWS` | `480` | Video height (pixels) |
| `SOURCE_FPS` | `30` | Frame rate |
| `SOURCE_FORMAT` | `RGB` | Output format (RGB, BGR, GRAY8, etc.) |

### videotestsrc Properties

| Environment Variable | Default Value | Description |
|---------|-------------|------|
| `PATTERN` | `0` | Test pattern (0=smpte, 1=snow, 2=black, 3=white, 4=red, 5=green, 6=blue) |
| `ANIMATION_MODE` | `0` | Animation mode (0=frames, 1=wall-time, 2=running-time) |
| `MOTION` | `0` | Motion settings |
| `BACKGROUND_COLOR` | `0xff000000` | Background color (ARGB format) |
| `FOREGROUND_COLOR` | `0xffffffff` | Foreground color (ARGB format) |
| `FLIP` | `false` | Video flip |
| `IS_LIVE` | `true` | Live streaming mode |

## Usage Examples

### Run with Default Settings
```bash
cargo run
```

### Run with Custom Settings
```bash
# HD resolution, 60FPS, snow noise pattern
IMAGE_COLS=1920 IMAGE_ROWS=1080 SOURCE_FPS=60 PATTERN=1 cargo run

# Solid color (red) pattern
PATTERN=4 cargo run

# Output in BGR format
SOURCE_FORMAT=BGR cargo run
```

## Build

```bash
cargo build --release
```

## Dependencies

- `dora-node-api`: Integration with dora-rs
- `kornia-io`: GStreamer integration and video processing
- GStreamer: Must be installed on the system

### Installing GStreamer

#### Ubuntu/Debian
```bash
sudo apt install gstreamer1.0-tools gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad
```

#### macOS
```bash
brew install gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad
```

## Test Pattern Types

- `0`: SMPTE color bar test
- `1`: Random noise (snow)
- `2`: Black screen
- `3`: White screen  
- `4`: Red screen
- `5`: Green screen
- `6`: Blue screen
- `7`: Checkerboard
- `8`: Horizontal gradient
- Many other patterns available

## Output

The node outputs video frames with the name `frame`. Each frame includes the following metadata:

- `encoding`: Format (RGB, BGR, etc.)
- `width`: Video width
- `height`: Video height

## Troubleshooting

### GStreamer Errors
- Check that GStreamer is properly installed
- Verify that required plugins are installed

### Format Errors
- Check that `SOURCE_FORMAT` is set to a valid value (RGB, BGR, GRAY8, etc.)

### Performance Issues
- Try reducing frame rate or size
- Check system resource usage 
