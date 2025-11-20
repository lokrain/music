use music_time::{TimePoint, TimeSpan};
use proptest::prelude::*;

fn approx_eq(lhs: f64, rhs: f64) -> bool {
    let delta = (lhs - rhs).abs();
    let scale = 1.0 + lhs.abs().max(rhs.abs());
    delta <= 1e-9 * scale
}

proptest! {
    #[test]
    fn timepoint_addition_is_associative(start in 0.0f64..=10_000.0, span_a in 0.0f64..=5_000.0, span_b in 0.0f64..=5_000.0) {
        let start = TimePoint::new(start);
        let span_a = TimeSpan::new(span_a);
        let span_b = TimeSpan::new(span_b);

        let left = (start + span_a) + span_b;
        let right = start + (span_a + span_b);
        prop_assert!(approx_eq(left.as_beats(), right.as_beats()));
    }

    #[test]
    fn timepoint_add_sub_roundtrip(base in 0.0f64..=20_000.0, span in 0.0f64..=10_000.0) {
        let start = TimePoint::new(base + span + 1.0);
        let span = TimeSpan::new(span);
        let after = start + span;
        let back = after - span;
        prop_assert!(approx_eq(start.as_beats(), back.as_beats()));
    }
}
