use crate::read::AiffData;

pub fn calculate_bpm_energy_manual_threshold(aiff_data: &AiffData, window_size: usize, threshold: f32) -> Result<f32, Box<dyn std::error::Error>> {
    /* Calculates the BPM of the song using a set energy calculation and threshold.
    Energy defined as the sum of each squared value in the window. Peaks detected if
    energy in a window is greater than the threshold. */

    let reference_audio_data: Vec<f32> = if aiff_data.num_channels == 1 {
        aiff_data.left_channel_audio.clone()
    } else {
        aiff_data.left_channel_audio  // reference audio is the average of left and right channels for best normalization
            .iter()
            .zip(aiff_data.right_channel_audio.iter())
            .map(|(&left, &right)| (left + right) / 2.0)
            .collect()
    };

    let mut energies: Vec<f32> = reference_audio_data.chunks(window_size)  // calculating the energy for each window
        .map(|window| window.iter().map(|&x| x * x).sum::<f32>())
        .collect();

    let max_energy = *energies.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();  // normalize the energies from 0 to 1
    for energy in &mut energies {
        *energy /= max_energy;
    }

    let mut peaks: Vec<usize> = Vec::new();  // peaks is a list of indices of where the peaks are
    for i in 1..energies.len() - 1 {  // finding energy peaks in the energies (if it's above the threshold, and higher than its surrounding energies)
        if energies[i] > threshold && energies[i] > energies[i-1] && energies[i] > energies[i+1] {
            peaks.push(i);
        }
    }

    let mut diffs: Vec<f32> = Vec::new();
    for i in 1..peaks.len() {  // calculate time difference between peaks
        let diff = (peaks[i] - peaks[i-1]) as f32 * window_size as f32 / aiff_data.sample_rate_hz as f32;
        diffs.push(diff);
    }

    let avg_diff = diffs.iter().sum::<f32>() / diffs.len() as f32;  // time difference average between peaks (in seconds)

    Ok(60.0 / avg_diff)  // return the bpm
}

pub fn calculate_bpm_energy_dynamic_threshold(aiff_data: &AiffData, window_size: usize, stddev_multiple: f32) -> Result<f32, Box<dyn std::error::Error>> {
    /* Calculates the BPM using energy with a sliding window, but dynamically sets the threshold.
    Algorithm:
    1. Average audio channels together if stereo.
    2. Calculate energies for the defined window.
    3. Normalize energies from 0 to 1.
    4. Calculate the average and standard deviation of the energies across all windows.
    5. Define threshold based on the following: avg + (x * sttdev), where x is some multiple of the stddev.
    6. Find all peaks with this dynamic threshold.
    7. Calculate the BPM.
    */

    let reference_audio_data: Vec<f32> = if aiff_data.num_channels == 1 {
        aiff_data.left_channel_audio.clone()
    } else {
        aiff_data.left_channel_audio
            .iter()
            .zip(aiff_data.right_channel_audio.iter())
            .map(|(&left, &right)| (left + right) / 2.0)
            .collect()
    };

    let mut energies: Vec<f32> = reference_audio_data.chunks(window_size)
        .map(|window| window.iter().map(|&x| x * x).sum::<f32>())
        .collect();

    let max_energy = *energies.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    for energy in &mut energies {
        *energy /= max_energy;
    }

    let avg_energy = energies.iter().sum::<f32>() / energies.len() as f32;
    let variance = energies.iter().map(|&x| (x - avg_energy).powi(2)).sum::<f32>() / energies.len() as f32;
    let stddev = variance.sqrt();

    let threshold = avg_energy + (stddev_multiple * stddev);

    let mut peaks: Vec<usize> = Vec::new();
    for i in 1..energies.len() - 1 {
        if energies[i] > threshold && energies[i] > energies[i-1] && energies[i] > energies[i+1] {
            peaks.push(i);
        }
    }

    let mut diffs: Vec<f32> = Vec::new();
    for i in 1..peaks.len() {
        let diff = (peaks[i] - peaks[i-1]) as f32 * window_size as f32 / aiff_data.sample_rate_hz as f32;
        diffs.push(diff);
    }

    if diffs.is_empty() {
        return Err("No peaks detected. Try adjusting the window size or stddev multiple.".into());
    }

    let avg_diff = diffs.iter().sum::<f32>() / diffs.len() as f32;

    Ok(60.0 / avg_diff)
}
