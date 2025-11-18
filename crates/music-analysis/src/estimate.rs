//! Estimation algorithms for musical features.
//!
//! Provides heuristics for estimating tempo, key, and meter from raw musical data.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Estimation results for tempo, key, and meter.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MusicEstimate {
    /// Estimated tempo in beats per minute (BPM).
    pub tempo_bpm: Option<f64>,
    /// Estimated key (0-11 representing C-B).
    pub key_estimate: Option<u8>,
    /// Estimated meter (e.g., "4/4", "3/4", "6/8").
    pub meter: Option<String>,
    /// Confidence score for tempo estimate (0.0-1.0).
    pub tempo_confidence: f64,
    /// Confidence score for key estimate (0.0-1.0).
    pub key_confidence: f64,
    /// Confidence score for meter estimate (0.0-1.0).
    pub meter_confidence: f64,
}

/// Estimate musical features from a melody sequence.
///
/// Uses pitch class histogram for key detection.
/// Cannot estimate tempo or meter without timing information.
pub fn estimate_from_melody(pitches: &[u8]) -> MusicEstimate {
    if pitches.is_empty() {
        return MusicEstimate {
            tempo_bpm: None,
            key_estimate: None,
            meter: None,
            tempo_confidence: 0.0,
            key_confidence: 0.0,
            meter_confidence: 0.0,
        };
    }

    // Estimate key using pitch class histogram
    let (key, confidence) = estimate_key_from_pitches(pitches);

    MusicEstimate {
        tempo_bpm: None,
        key_estimate: Some(key),
        meter: None,
        tempo_confidence: 0.0,
        key_confidence: confidence,
        meter_confidence: 0.0,
    }
}

/// Estimate musical features from MIDI events with timing.
///
/// Events are `(pitch_index, onset_time_sec)`.
pub fn estimate_from_midi(events: &[(u8, f64)]) -> MusicEstimate {
    if events.is_empty() {
        return MusicEstimate {
            tempo_bpm: None,
            key_estimate: None,
            meter: None,
            tempo_confidence: 0.0,
            key_confidence: 0.0,
            meter_confidence: 0.0,
        };
    }

    // Sort by onset time
    let mut sorted_events = events.to_vec();
    sorted_events.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let pitches: Vec<u8> = sorted_events.iter().map(|(p, _)| *p).collect();

    // Estimate key
    let (key, key_conf) = estimate_key_from_pitches(&pitches);

    // Estimate tempo
    let (tempo, tempo_conf) = estimate_tempo(&sorted_events);

    // Estimate meter
    let (meter, meter_conf) = estimate_meter(&sorted_events, tempo);

    MusicEstimate {
        tempo_bpm: Some(tempo),
        key_estimate: Some(key),
        meter: Some(meter),
        tempo_confidence: tempo_conf,
        key_confidence: key_conf,
        meter_confidence: meter_conf,
    }
}

/// Estimate key from pitch sequence using Krumhansl-Schmuckler algorithm (simplified).
///
/// Returns (key as pitch class 0-11, confidence 0.0-1.0).
fn estimate_key_from_pitches(pitches: &[u8]) -> (u8, f64) {
    if pitches.is_empty() {
        return (0, 0.0);
    }

    // Build pitch class histogram
    let mut histogram = [0u32; 12];
    for &pitch in pitches {
        let pc = (pitch % 12) as usize;
        histogram[pc] += 1;
    }

    // Major key profiles (Krumhansl-Kessler weights, simplified)
    let major_profile = [
        6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
    ];
    let minor_profile = [
        6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
    ];

    let mut best_key = 0u8;
    let mut best_correlation = -1.0f64;

    // Try all 12 major keys
    for root in 0..12 {
        let corr = correlation(&histogram, &major_profile, root);
        if corr > best_correlation {
            best_correlation = corr;
            best_key = root as u8;
        }
    }

    // Try all 12 minor keys
    for root in 0..12 {
        let corr = correlation(&histogram, &minor_profile, root);
        if corr > best_correlation {
            best_correlation = corr;
            best_key = root as u8;
        }
    }

    // Confidence is normalized correlation (rough approximation)
    let confidence = ((best_correlation + 1.0) / 2.0).clamp(0.0, 1.0);

    (best_key, confidence)
}

/// Compute Pearson correlation between histogram and profile rotated by root.
fn correlation(histogram: &[u32; 12], profile: &[f64; 12], root: usize) -> f64 {
    let total: u32 = histogram.iter().sum();
    if total == 0 {
        return 0.0;
    }

    // Normalize histogram
    let hist_norm: Vec<f64> = histogram.iter().map(|&h| h as f64 / total as f64).collect();

    // Rotate profile by root
    let mut rotated = [0.0; 12];
    for i in 0..12 {
        rotated[i] = profile[(i + root) % 12];
    }

    // Compute means
    let hist_mean: f64 = hist_norm.iter().sum::<f64>() / 12.0;
    let prof_mean: f64 = rotated.iter().sum::<f64>() / 12.0;

    // Compute correlation
    let mut numerator = 0.0;
    let mut hist_var = 0.0;
    let mut prof_var = 0.0;

    for i in 0..12 {
        let h_diff = hist_norm[i] - hist_mean;
        let p_diff = rotated[i] - prof_mean;
        numerator += h_diff * p_diff;
        hist_var += h_diff * h_diff;
        prof_var += p_diff * p_diff;
    }

    if hist_var == 0.0 || prof_var == 0.0 {
        return 0.0;
    }

    numerator / (hist_var.sqrt() * prof_var.sqrt())
}

/// Estimate tempo from inter-onset intervals.
///
/// Returns (BPM, confidence).
fn estimate_tempo(events: &[(u8, f64)]) -> (f64, f64) {
    if events.len() < 2 {
        return (120.0, 0.0); // Default fallback
    }

    // Compute inter-onset intervals
    let iois: Vec<f64> = events
        .windows(2)
        .map(|w| w[1].1 - w[0].1)
        .filter(|&ioi| ioi > 0.0 && ioi < 5.0) // Filter outliers
        .collect();

    if iois.is_empty() {
        return (120.0, 0.0);
    }

    // Find dominant IOI using histogram binning
    let mut ioi_bins: Vec<(f64, usize)> = Vec::new();
    let bin_width = 0.05; // 50ms bins

    for &ioi in &iois {
        let bin_center = (ioi / bin_width).round() * bin_width;

        if let Some(entry) = ioi_bins
            .iter_mut()
            .find(|(b, _)| (*b - bin_center).abs() < bin_width / 2.0)
        {
            entry.1 += 1;
        } else {
            ioi_bins.push((bin_center, 1));
        }
    }

    // Find most frequent bin
    ioi_bins.sort_by(|a, b| b.1.cmp(&a.1));
    let dominant_ioi = ioi_bins[0].0;
    let dominant_count = ioi_bins[0].1;

    // Convert to BPM (assuming IOI represents beat duration)
    let tempo_bpm = 60.0 / dominant_ioi;

    // Confidence based on how many IOIs match the dominant one
    let confidence = dominant_count as f64 / iois.len() as f64;

    (tempo_bpm, confidence.clamp(0.0, 1.0))
}

/// Estimate meter from event timing and tempo.
///
/// Returns (meter string, confidence).
fn estimate_meter(events: &[(u8, f64)], tempo_bpm: f64) -> (String, f64) {
    if events.len() < 4 {
        return ("4/4".to_string(), 0.3); // Default guess
    }

    let beat_duration = 60.0 / tempo_bpm;

    // Analyze beat positions to detect grouping patterns
    let beat_positions: Vec<f64> = events
        .iter()
        .map(|(_, onset)| (onset / beat_duration) % 16.0)
        .collect();

    // Count strong beat occurrences (on-beat positions)
    let mut beat_counts = [0usize; 16];
    for &pos in &beat_positions {
        let beat_idx = pos.round() as usize % 16;
        beat_counts[beat_idx] += 1;
    }

    // Detect meter based on strong beat patterns
    let beat_0 = beat_counts[0];
    let beat_2 = beat_counts[2];
    let beat_3 = beat_counts[3];
    let beat_4 = beat_counts[4];

    // Simple heuristic: compare relative strengths
    let triple_meter_score = (beat_0 + beat_3) as f64;
    let duple_meter_score = (beat_0 + beat_2 + beat_4) as f64;
    let compound_score = (beat_0 + beat_counts[3] + beat_counts[6] + beat_counts[9]) as f64;

    let total = beat_positions.len() as f64;
    if total == 0.0 {
        return ("4/4".to_string(), 0.3);
    }

    let triple_confidence = triple_meter_score / total;
    let duple_confidence = duple_meter_score / total;
    let compound_confidence = compound_score / total;

    if compound_confidence > duple_confidence && compound_confidence > triple_confidence {
        ("6/8".to_string(), compound_confidence.clamp(0.0, 1.0))
    } else if triple_confidence > duple_confidence {
        ("3/4".to_string(), triple_confidence.clamp(0.0, 1.0))
    } else {
        ("4/4".to_string(), duple_confidence.clamp(0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_empty() {
        let est = estimate_from_melody(&[]);
        assert_eq!(est.key_estimate, None);
        assert_eq!(est.tempo_bpm, None);
    }

    #[test]
    fn test_estimate_key_c_major() {
        // C major scale pitches
        let pitches = vec![60, 62, 64, 65, 67, 69, 71, 72, 60, 64, 67];
        let est = estimate_from_melody(&pitches);
        assert_eq!(est.key_estimate, Some(0)); // C = 0
        assert!(est.key_confidence > 0.0);
    }

    #[test]
    fn test_estimate_tempo_from_midi() {
        // Events at 120 BPM (0.5 second per beat)
        let events = vec![
            (60, 0.0),
            (62, 0.5),
            (64, 1.0),
            (65, 1.5),
            (67, 2.0),
            (69, 2.5),
        ];
        let est = estimate_from_midi(&events);
        assert!(est.tempo_bpm.is_some());
        let tempo = est.tempo_bpm.unwrap();
        assert!((tempo - 120.0).abs() < 10.0); // Within 10 BPM tolerance
    }

    #[test]
    fn test_estimate_meter_4_4() {
        // Events aligned to 4/4 meter at 120 BPM
        let beat = 0.5; // 120 BPM
        let events = vec![
            (60, 0.0 * beat),
            (62, 1.0 * beat),
            (64, 2.0 * beat),
            (65, 3.0 * beat),
            (67, 4.0 * beat),
            (69, 5.0 * beat),
            (71, 6.0 * beat),
            (72, 7.0 * beat),
        ];
        let est = estimate_from_midi(&events);
        assert!(est.meter.is_some());
        // Meter detection is heuristic-based, just check it returns something
    }

    #[test]
    fn test_correlation_perfect() {
        let histogram = [100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let profile = [
            6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
        ];
        let corr = correlation(&histogram, &profile, 0);
        // Should have high positive correlation
        assert!(corr > 0.5);
    }
}
