mod reader;
use reader::read_aiff;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("failed to get current directory");
    let file_path = current_dir.join("samples").join("test.aiff").to_string_lossy().into_owned();

    println!("Reading file: {}", file_path);
    println!("{}", "-".repeat(75));

    let data = match read_aiff(&file_path) {
        Ok(aiff_data) => aiff_data,
        Err(err) => {
            eprintln!("Error reading AIFF file: {}", err);
            return;
        }
    };

    println!("File Size: {} bytes", data.file_size);
    println!("Num Channels: {}", data.num_channels);
    println!("Num Sample Frames: {}", data.num_sample_frames);
    println!("Sample Size: {}", data.sample_size);
    println!("Sample Rate: {} hz", data.sample_rate);
    println!("Track Name: {}", data.track_name);
    println!("Track Length: {} seconds", data.track_length);
}
