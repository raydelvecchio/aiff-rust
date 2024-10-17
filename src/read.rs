use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt};
use std::time::Instant;

const MAX_F32_SIZE: f32 = 32768.0;

pub struct AiffData {
    pub file_size_bytes: u32,
    pub num_channels: u16,
    pub num_sample_frames: u32,
    pub bit_depth: u16,
    pub sample_rate_hz: u32,
    pub track_name: String,
    pub track_length_s: u16,
    pub sound_offset_bytes: u16,
    pub sound_block_size_bytes: u16,
    pub left_channel_audio: Vec<f32>,  // all audio data is stored as an f32, where all values are scaled from -1 to 1
    pub right_channel_audio: Vec<f32>,
    pub interleaved_audio: Vec<f32>,
}

pub fn read_aiff(filepath: &str) -> Result<AiffData, Box<dyn std::error::Error>> {
    /* Reads the .aiff file and prints key information about it. */

    let start_time = Instant::now();

    let mut file = File::open(filepath)?;  // opens the file at the filepath, the ? instantly propogates the result or error to the current scope (so we don't have to manually handle Ok and Err with a match)

    let mut form_chunk = [0u8; 4];  // first 4 bytes are the FORM chunk
    file.read_exact(&mut form_chunk)?;  // reads from the file the amount of bytes to fill up buf (in this case, it's an array of four u8 zeros declared a line above)
    if &form_chunk != b"FORM" {
        return Err("Not a valid AIFF file".into());
    }

    let mut file_size_bytes = [0u8; 4];  // next 4 bytes are the file size
    file.read_exact(&mut file_size_bytes)?;  // mutable borrow of the variable file_size_bytes, allowing it to be modified in the function (rust allows you to control editing / mutating of variables well like this!)
    let file_size = u32::from_be_bytes(file_size_bytes) + 8;

    let mut aiff_id = [0u8; 4];  // 4 bytes after that are the AIFF identifier
    file.read_exact(&mut aiff_id)?;
    if &aiff_id != b"AIFF" {
        return Err("Not a valid AIFF file".into());
    }

    let mut name_chunk = [0u8; 4];  // 4 bytes after the AIFF identifier is the NAME chunk (which is optional)
    let mut track_name = String::new();
    file.read_exact(&mut name_chunk)?;
    if &name_chunk == b"NAME" {
        let chunk_size = file.read_u32::<BigEndian>()?;
        let mut name_data = vec![0u8; chunk_size as usize];
        file.read_exact(&mut name_data)?;
        track_name = String::from_utf8_lossy(&name_data).to_string();
    } else {
        file.seek(SeekFrom::Current(-4))?;  // go back 4 bytes if this isn't the name chunk
    }

    let mut comm_chunk = [0u8; 4];  // next 4 bytes is the COMM chunk
    file.read_exact(&mut comm_chunk)?;
    if &comm_chunk != b"COMM" {
        return Err("COMM chunk not found".into());
    }

    let comm_chunk_size = file.read_u32::<BigEndian>()?;  // after the COMM chunk, verify that the size is 18 (by reading next 4 bytes) 
    if comm_chunk_size != 18 {
        return Err("Unexpected COMM chunk size".into());
    }

    let num_channels = file.read_u16::<BigEndian>()?;  // 2 bytes for channel count
    let num_sample_frames = file.read_u32::<BigEndian>()?;  // 4 bytes for number of frames
    let bit_depth = file.read_u16::<BigEndian>()?;  // 2 bytes for bit depth
    let sample_rate_hz = read_extended_float(&mut file)? as u32;  // 10 bytes for sample rate
    let track_length_s = (num_sample_frames / sample_rate_hz) as u16;

    let mut ssnd_chunk = [0u8; 4];  // 4 bytes after this is SSND chunk
    file.read_exact(&mut ssnd_chunk)?;
    if &ssnd_chunk != b"SSND" {
        return Err("SSND chunk not found".into());
    }

    let _ssnd_chunk_size = file.read_u32::<BigEndian>()?;  // chunk size 4 bytes
    let ssnd_offset = file.read_u16::<BigEndian>()?;  // ssnd offset 4 bytes
    let ssnd_block_size = file.read_u16::<BigEndian>()?;  // block size 4 bytes

    file.seek(SeekFrom::Current(ssnd_offset as i64))?;  // respect the ssnd offset

    let mut audio_data = Vec::new();
    file.read_to_end(&mut audio_data)?;  // reading all of the audio data into a buffer to the end of the file

    let mut left_channel_audio = Vec::new();
    let mut right_channel_audio = Vec::new();
    let mut interleaved_audio = Vec::new();
    let bytes_per_sample = bit_depth as usize / 8;

    if num_channels == 1 {
        left_channel_audio = audio_data.chunks(bytes_per_sample)
            .map(|chunk| (i16::from_be_bytes([chunk[0], chunk[1]]) as f32) / MAX_F32_SIZE)
            .collect();

        right_channel_audio = left_channel_audio.clone();
        interleaved_audio = left_channel_audio.clone();
    } else if num_channels == 2 {
        for chunk in audio_data.chunks(bytes_per_sample * 2) {
            if chunk.len() == bytes_per_sample * 2 {
                let left_sample_int = i16::from_be_bytes([chunk[0], chunk[1]]);  // first load as an i16
                let right_sample_int = i16::from_be_bytes([chunk[2], chunk[3]]);

                let left_sample_f32 = (left_sample_int as f32) / MAX_F32_SIZE;  // convert to f32 for better sampling and audio inference
                let right_sample_f32 = (right_sample_int as f32) / MAX_F32_SIZE;

                left_channel_audio.push(left_sample_f32);
                right_channel_audio.push(right_sample_f32);
                interleaved_audio.push(left_sample_f32);
                interleaved_audio.push(right_sample_f32);
            }
        }
    } else {
        return Err("Must have either 1 or 2 audio channels".into());
    }

    if left_channel_audio.len() != right_channel_audio.len() {
        return Err("Left and right audio channels have unequal length".into());
    }

    let duration = start_time.elapsed();

    print!("{} loaded in {}ms\n", filepath, duration.as_millis());

    Ok(AiffData {
        file_size_bytes: file_size,
        num_channels,
        num_sample_frames,
        bit_depth,
        sample_rate_hz,
        track_name,
        track_length_s,
        sound_offset_bytes: ssnd_offset,
        sound_block_size_bytes: ssnd_block_size,
        left_channel_audio,
        right_channel_audio,
        interleaved_audio,
    })
}

fn read_extended_float(file: &mut File) -> Result<f64, Box<dyn std::error::Error>> {
    /* aiff files use 80-bit (10-byte) floating point to store the sample rate. this isn't supported
    natively by rust. Thus, we have to parse it ourselves. We pass in the file, assuming the
    pointer is right at the starting point of the 10 bytes we want to read. */
    let mut buffer = [0u8; 10];
    file.read_exact(&mut buffer)?;

    let sign = if buffer[0] & 0x80 != 0 { -1.0 } else { 1.0 };
    let exponent = ((buffer[0] as u16 & 0x7F) << 8 | buffer[1] as u16) as i16 - 16383;
    let fraction = (buffer[2] as u64) << 56 | (buffer[3] as u64) << 48 |
                   (buffer[4] as u64) << 40 | (buffer[5] as u64) << 32 |
                   (buffer[6] as u64) << 24 | (buffer[7] as u64) << 16 |
                   (buffer[8] as u64) << 8  | (buffer[9] as u64);

    Ok(sign * (fraction as f64) * 2f64.powi(exponent as i32 - 63))
}
