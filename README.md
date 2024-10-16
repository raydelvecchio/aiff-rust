# aiff.rs
Processing of `.aiff` files in Rust.

# Useful Commands
* `cargo run`: compiles and runs the main file of the project (usually `src/main.rs`)
* `rustc FILE.rs`: complies a specific file to run as an executable, which you can then run with `./PATH_TO_FILE`
* `cargo add PACKAGE`: adds the package to Cargo, thus, when compiling, we have access to that package
* `cargo build`: compils the rust project and dependencies into `/target/debug`
* `cargo build --release` is used to build a release ready version without debug flagging

# Notes
* The .aiff file format
    * Contains 2 stereo channels of audio
    * Usually sampled at *44.1khz* (44,100 *frame rate*)
        * 44,100 samples of audio taken every second on converstion / recording
        * Smoother audio data
    * 16 bit *sample size*, AKA *bit depth*
        * This means that every sample of audio is represented with 16 bits
        * This can increase to 24 bits, which will have higher fidelity data, but increase file size
        * *sample width* is the same thing, but in bytes: a 16 bit depth means a sample width of 2!
    * Structure
        * First 4 bytes: the `FORM` chunk
        * Next 4 bytes: File size (in bytes, big-endian)
        * Next 4 bytes: AIFF identifier ("AIFF")
        * NAME chunk (optional):
            * 4 bytes: "NAME" identifier
            * 4 bytes: Chunk size
            * Variable: Text data (file name)
        * COMM chunk:
            * 4 bytes: "COMM" identifier
            * 4 bytes: Chunk size (18 for standard AIFF)
            * 2 bytes: Number of channels
            * 4 bytes: Number of sample frames
            * 2 bytes: Sample size (bits per sample)
            * 10 bytes: Sample rate (80-bit IEEE 754 extended floating-point)
        * SSND chunk:
            * 4 bytes: "SSND" identifier
            * 4 bytes: Chunk size
            * 4 bytes: Offset
            * 4 bytes: Block size
            * Remaining: Audio data
* Spotify API features:
    * Acousticness: Calculated using spectral centroids. Higher values indicate more acoustic sound.
    * Danceability: Derived from zero crossing rate. Reflects rhythm stability and beat strength.
    * Energy: Computed from spectral rolloff. Represents the overall energy of the track.
    * Instrumentalness: Based on the first MFCC coefficient. Higher values suggest more instrumental content.
    * Liveness: Determined by the maximum RMS energy. Indicates presence of audience in the recording.
    * Loudness: Converted from mean RMS energy to decibels. Represents overall volume of the track.
    * Speechiness: Calculated using the zeroth MFCC coefficient. Detects presence of spoken words.
    * Valence: Derived from spectral contrast. Indicates musical positiveness conveyed by the track.
    * Tempo: Estimated using beat detection algorithms. Measured in beats per minute (BPM).
    * Key: Determined by the dominant chroma feature. Represents the musical key of the track.
    * Mode: Calculated using tonnetz features. Indicates whether the track is in major (1) or minor (0) mode.
    * Time Signature: Estimated using tempogram analysis. Represents the number of beats in each bar.
    * Duration: Calculated from the total number of samples and sample rate. Measured in milliseconds.

# Error Log
* AIFF identifier contains extended characters and thus is invalid
    * The file format of AIFF gets fucked up if you open it in a text editor, and then save the file
    * This adds some unnecessary stuff in there that messes up the file format and makes it unreadable
    * DO NOT open a `.aiff` in a text editor if you want to process it at any point later
