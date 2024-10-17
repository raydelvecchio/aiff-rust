use rodio::{OutputStream, Sink};
use crate::read::AiffData;

pub fn play_audio(aiff_data: &AiffData) -> Result<(), Box<dyn std::error::Error>> {
    /* Plays the audio from a .aiff file using the stream of information from the AiffData struct. */

    let source = rodio::buffer::SamplesBuffer::new(  // new audio source, specifying channels and sample rate, as well as audio data
        aiff_data.num_channels,
        aiff_data.sample_rate_hz,
        aiff_data.interleaved_audio.clone()
    );

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
