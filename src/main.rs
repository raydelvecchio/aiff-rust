mod reader;
use reader::read_aiff;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().expect("failed to get current directory");
    let file_path = current_dir.join("samples").join("shiver.aiff").to_string_lossy().into_owned();

    let data = read_aiff(&file_path)?;

    println!("File Size: {} bytes", data.file_size_bytes);
    println!("Num Channels: {}", data.num_channels);
    println!("Num Sample Frames: {}", data.num_sample_frames);
    println!("Bit Depth: {}", data.bit_depth);
    println!("Sample Rate: {} hz", data.sample_rate_hz);
    println!("Track Name: {}", data.track_name);
    println!("Track Length: {} seconds", data.track_length_s);
    println!("Sound Offset: {} bytes", data.sound_offset_bytes);
    println!("Sound Block Size: {} bytes", data.sound_block_size_bytes);

    Ok(())
}
