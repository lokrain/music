//! notation/music-articulation/src/dynamics.rs
//! Dynamics and hairpins.

/// Dynamic markings ordered from softest to loudest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DynamicMark {
    Ppp = 0,
    Pp = 1,
    P = 2,
    Mp = 3,
    Mf = 4,
    F = 5,
    Ff = 6,
    Fff = 7,
}

impl DynamicMark {
    /// Human-readable shorthand (e.g. "mf").
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            DynamicMark::Ppp => "ppp",
            DynamicMark::Pp => "pp",
            DynamicMark::P => "p",
            DynamicMark::Mp => "mp",
            DynamicMark::Mf => "mf",
            DynamicMark::F => "f",
            DynamicMark::Ff => "ff",
            DynamicMark::Fff => "fff",
        }
    }

    /// Intensity rank (0 softest, 7 loudest).
    #[must_use]
    pub const fn intensity(self) -> u8 {
        self as u8
    }

    /// Convert an intensity rank back into a [`DynamicMark`].
    #[must_use]
    pub fn from_intensity(intensity: u8) -> Option<Self> {
        match intensity {
            0 => Some(DynamicMark::Ppp),
            1 => Some(DynamicMark::Pp),
            2 => Some(DynamicMark::P),
            3 => Some(DynamicMark::Mp),
            4 => Some(DynamicMark::Mf),
            5 => Some(DynamicMark::F),
            6 => Some(DynamicMark::Ff),
            7 => Some(DynamicMark::Fff),
            _ => None,
        }
    }
}

/// Hairpin shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hairpin {
    Crescendo,
    Decrescendo,
}

/// Keyframe describing a dynamic mark at a relative offset (beats or seconds).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DynamicProfilePoint {
    pub offset: f64,
    pub mark: DynamicMark,
}

/// Helpers for building simple dynamic profiles.
pub struct DynamicProfile;

impl DynamicProfile {
    /// Evenly distribute a linear ramp between two dynamics over `duration` units.
    ///
    /// Returns `steps` keyframes including both endpoints.
    #[must_use]
    pub fn ramp(
        start: DynamicMark,
        end: DynamicMark,
        duration: f64,
        steps: usize,
    ) -> Vec<DynamicProfilePoint> {
        assert!(steps >= 2, "ramp requires at least two steps");
        assert!(
            duration >= 0.0 && duration.is_finite(),
            "duration must be non-negative and finite"
        );
        let mut points = Vec::with_capacity(steps);
        let start_intensity = start.intensity() as f64;
        let end_intensity = end.intensity() as f64;
        let denom = (steps - 1) as f64;
        for i in 0..steps {
            let t = i as f64 / denom;
            let offset = duration * t;
            let intensity = start_intensity + (end_intensity - start_intensity) * t;
            let mark = DynamicMark::from_intensity(intensity.round() as u8).expect("valid dynamic");
            points.push(DynamicProfilePoint { offset, mark });
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_matches_intensity() {
        assert!(DynamicMark::Ppp < DynamicMark::Pp);
        assert!(DynamicMark::Ff < DynamicMark::Fff);
        assert_eq!(DynamicMark::Mf.intensity(), 4);
        assert_eq!(DynamicMark::from_intensity(4), Some(DynamicMark::Mf));
    }

    #[test]
    fn ramp_profile_includes_endpoints() {
        let ramp = DynamicProfile::ramp(DynamicMark::P, DynamicMark::Ff, 4.0, 3);
        assert_eq!(ramp.len(), 3);
        assert_eq!(ramp.first().unwrap().mark, DynamicMark::P);
        assert_eq!(ramp.last().unwrap().mark, DynamicMark::Ff);
        assert!((ramp[1].offset - 2.0).abs() < 1e-9);
    }
}
