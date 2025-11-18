//! Profile analyzer for musical sequences.
//!
//! Computes timing statistics, note density, register usage, and percentile distributions
//! for melody and MIDI inputs.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Statistics summarizing profile metrics.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProfileStats {
    /// Total number of events (notes).
    pub event_count: usize,
    /// Total duration in seconds (if timing data available).
    pub total_duration_sec: Option<f64>,
    /// Note density: events per second (if timing available).
    pub density_events_per_sec: Option<f64>,
    /// Pitch range statistics.
    pub pitch_range: PitchRangeStats,
    /// Timing statistics (inter-onset intervals).
    pub timing: TimingStats,
}

/// Pitch range and register statistics.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PitchRangeStats {
    /// Minimum pitch index observed.
    pub min_pitch: Option<u8>,
    /// Maximum pitch index observed.
    pub max_pitch: Option<u8>,
    /// Median pitch index.
    pub median_pitch: Option<f64>,
    /// 25th percentile pitch.
    pub p25_pitch: Option<f64>,
    /// 75th percentile pitch.
    pub p75_pitch: Option<f64>,
}

/// Timing statistics for inter-onset intervals.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimingStats {
    /// Minimum inter-onset interval in seconds.
    pub min_ioi_sec: Option<f64>,
    /// Maximum inter-onset interval in seconds.
    pub max_ioi_sec: Option<f64>,
    /// Median inter-onset interval.
    pub median_ioi_sec: Option<f64>,
    /// 25th percentile IOI.
    pub p25_ioi_sec: Option<f64>,
    /// 75th percentile IOI.
    pub p75_ioi_sec: Option<f64>,
    /// Estimated swing ratio (if detectable).
    pub swing_ratio: Option<f64>,
}

/// Profile a melody sequence (pitch indices without timing).
///
/// Returns statistics summarizing pitch range and distribution.
/// Timing stats will be None since melody sequences lack timestamps.
pub fn profile_melody(pitches: &[u8]) -> ProfileStats {
    if pitches.is_empty() {
        return ProfileStats {
            event_count: 0,
            total_duration_sec: None,
            density_events_per_sec: None,
            pitch_range: PitchRangeStats {
                min_pitch: None,
                max_pitch: None,
                median_pitch: None,
                p25_pitch: None,
                p75_pitch: None,
            },
            timing: TimingStats {
                min_ioi_sec: None,
                max_ioi_sec: None,
                median_ioi_sec: None,
                p25_ioi_sec: None,
                p75_ioi_sec: None,
                swing_ratio: None,
            },
        };
    }

    let mut sorted = pitches.to_vec();
    sorted.sort_unstable();

    let min_pitch = Some(*sorted.first().unwrap());
    let max_pitch = Some(*sorted.last().unwrap());
    let median_pitch = Some(percentile(&sorted, 50.0));
    let p25_pitch = Some(percentile(&sorted, 25.0));
    let p75_pitch = Some(percentile(&sorted, 75.0));

    ProfileStats {
        event_count: pitches.len(),
        total_duration_sec: None,
        density_events_per_sec: None,
        pitch_range: PitchRangeStats {
            min_pitch,
            max_pitch,
            median_pitch,
            p25_pitch,
            p75_pitch,
        },
        timing: TimingStats {
            min_ioi_sec: None,
            max_ioi_sec: None,
            median_ioi_sec: None,
            p25_ioi_sec: None,
            p75_ioi_sec: None,
            swing_ratio: None,
        },
    }
}

/// Profile a MIDI-like sequence with timing information.
///
/// Each event is `(pitch_index, onset_time_sec)`.
pub fn profile_midi(events: &[(u8, f64)]) -> ProfileStats {
    if events.is_empty() {
        return ProfileStats {
            event_count: 0,
            total_duration_sec: None,
            density_events_per_sec: None,
            pitch_range: PitchRangeStats {
                min_pitch: None,
                max_pitch: None,
                median_pitch: None,
                p25_pitch: None,
                p75_pitch: None,
            },
            timing: TimingStats {
                min_ioi_sec: None,
                max_ioi_sec: None,
                median_ioi_sec: None,
                p25_ioi_sec: None,
                p75_ioi_sec: None,
                swing_ratio: None,
            },
        };
    }

    // Sort by onset time
    let mut sorted_events = events.to_vec();
    sorted_events.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let pitches: Vec<u8> = sorted_events.iter().map(|(p, _)| *p).collect();
    let mut sorted_pitches = pitches.clone();
    sorted_pitches.sort_unstable();

    let min_pitch = Some(*sorted_pitches.first().unwrap());
    let max_pitch = Some(*sorted_pitches.last().unwrap());
    let median_pitch = Some(percentile(&sorted_pitches, 50.0));
    let p25_pitch = Some(percentile(&sorted_pitches, 25.0));
    let p75_pitch = Some(percentile(&sorted_pitches, 75.0));

    // Compute timing stats from inter-onset intervals
    let total_duration_sec = if sorted_events.len() > 1 {
        Some(sorted_events.last().unwrap().1 - sorted_events.first().unwrap().1)
    } else {
        None
    };

    let density_events_per_sec = total_duration_sec.and_then(|dur| {
        if dur > 0.0 {
            Some(sorted_events.len() as f64 / dur)
        } else {
            None
        }
    });

    let iois: Vec<f64> = sorted_events.windows(2).map(|w| w[1].1 - w[0].1).collect();

    let timing = if !iois.is_empty() {
        let mut sorted_iois = iois.clone();
        sorted_iois.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        TimingStats {
            min_ioi_sec: Some(*sorted_iois.first().unwrap()),
            max_ioi_sec: Some(*sorted_iois.last().unwrap()),
            median_ioi_sec: Some(percentile_f64(&sorted_iois, 50.0)),
            p25_ioi_sec: Some(percentile_f64(&sorted_iois, 25.0)),
            p75_ioi_sec: Some(percentile_f64(&sorted_iois, 75.0)),
            swing_ratio: detect_swing(&iois),
        }
    } else {
        TimingStats {
            min_ioi_sec: None,
            max_ioi_sec: None,
            median_ioi_sec: None,
            p25_ioi_sec: None,
            p75_ioi_sec: None,
            swing_ratio: None,
        }
    };

    ProfileStats {
        event_count: sorted_events.len(),
        total_duration_sec,
        density_events_per_sec,
        pitch_range: PitchRangeStats {
            min_pitch,
            max_pitch,
            median_pitch,
            p25_pitch,
            p75_pitch,
        },
        timing,
    }
}

/// Compute percentile for u8 values using linear interpolation.
fn percentile(sorted: &[u8], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0] as f64;
    }

    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let frac = rank - lower as f64;

    sorted[lower] as f64 * (1.0 - frac) + sorted[upper] as f64 * frac
}

/// Compute percentile for f64 values using linear interpolation.
fn percentile_f64(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }

    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let frac = rank - lower as f64;

    sorted[lower] * (1.0 - frac) + sorted[upper] * frac
}

/// Attempt to detect swing ratio from inter-onset intervals.
///
/// Returns a swing ratio if a clear alternating pattern is detected.
/// Swing ratio > 1.0 indicates longer first beat, < 1.0 indicates shorter first beat.
fn detect_swing(iois: &[f64]) -> Option<f64> {
    if iois.len() < 6 {
        return None; // Need enough data
    }

    // Check for alternating pattern: group into pairs and compute ratio
    let pairs: Vec<(f64, f64)> = iois
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .collect();

    if pairs.len() < 3 {
        return None;
    }

    // Compute average ratio
    let ratios: Vec<f64> = pairs
        .iter()
        .filter_map(|(a, b)| if *b > 0.0 { Some(a / b) } else { None })
        .collect();

    if ratios.is_empty() {
        return None;
    }

    let avg_ratio: f64 = ratios.iter().sum::<f64>() / ratios.len() as f64;

    // Check if ratio is consistent (low variance)
    let variance: f64 = ratios
        .iter()
        .map(|r| {
            let diff = r - avg_ratio;
            diff * diff
        })
        .sum::<f64>()
        / ratios.len() as f64;

    let std_dev = variance.sqrt();

    // Only report swing if consistent and significantly different from 1.0
    if std_dev < 0.2 && (avg_ratio - 1.0).abs() > 0.15 {
        Some(avg_ratio)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_empty_melody() {
        let stats = profile_melody(&[]);
        assert_eq!(stats.event_count, 0);
        assert_eq!(stats.pitch_range.min_pitch, None);
        assert_eq!(stats.total_duration_sec, None);
    }

    #[test]
    fn test_profile_melody_basic() {
        let pitches = vec![60, 62, 64, 65, 67];
        let stats = profile_melody(&pitches);
        assert_eq!(stats.event_count, 5);
        assert_eq!(stats.pitch_range.min_pitch, Some(60));
        assert_eq!(stats.pitch_range.max_pitch, Some(67));
        assert_eq!(stats.pitch_range.median_pitch, Some(64.0));
    }

    #[test]
    fn test_profile_midi_with_timing() {
        let events = vec![(60, 0.0), (62, 0.5), (64, 1.0), (65, 1.5), (67, 2.0)];
        let stats = profile_midi(&events);
        assert_eq!(stats.event_count, 5);
        assert_eq!(stats.total_duration_sec, Some(2.0));
        assert_eq!(stats.density_events_per_sec, Some(2.5));
        assert_eq!(stats.timing.median_ioi_sec, Some(0.5));
    }

    #[test]
    fn test_percentile_calculation() {
        let data = vec![60, 62, 64, 66, 68, 70, 72];
        assert_eq!(percentile(&data, 0.0), 60.0);
        assert_eq!(percentile(&data, 50.0), 66.0);
        assert_eq!(percentile(&data, 100.0), 72.0);
    }

    #[test]
    fn test_swing_detection_no_swing() {
        let iois = vec![0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
        assert_eq!(detect_swing(&iois), None);
    }

    #[test]
    fn test_swing_detection_with_swing() {
        // 2:1 swing pattern
        let iois = vec![0.66, 0.33, 0.66, 0.33, 0.66, 0.33];
        let swing = detect_swing(&iois);
        assert!(swing.is_some());
        let ratio = swing.unwrap();
        assert!((ratio - 2.0).abs() < 0.1);
    }
}
