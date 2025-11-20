use music_time::{TimePoint, meter::Meter, timegrid::GridConfig};
use proptest::prelude::*;

fn arb_meter() -> impl Strategy<Value = Meter> {
    (1u8..=12, prop_oneof![Just(2u8), Just(4u8), Just(8u8), Just(16u8)])
        .prop_map(|(numerator, denominator)| Meter::new(numerator, denominator))
}

fn strictly_increasing(points: &[TimePoint]) -> bool {
    points.windows(2).all(|window| window[1].as_beats() - window[0].as_beats() > -1e-9)
}

proptest! {
    #[test]
    fn grid_is_monotonic(
        start in 0.0f64..=128.0,
        meter in arb_meter(),
        bars in 1u32..=64,
        subdivisions in 1u32..=8,
    ) {
        let start = TimePoint::new(start);
        let grid = GridConfig::new(start, meter)
            .bars(bars)
            .subdivisions_per_beat(subdivisions)
            .build();

        prop_assert!(strictly_increasing(grid.measures()));
        prop_assert!(strictly_increasing(grid.beats()));
        prop_assert!(strictly_increasing(grid.subdivisions()));

        let total_span = grid.measures().last().unwrap().as_beats() - start.as_beats();
        let expected = meter.bar_span().as_beats() * f64::from(bars);
        prop_assert!((total_span - expected).abs() <= 1e-6 * (1.0 + expected.abs()));
    }

    #[test]
    fn grids_align_across_meter_changes(
        start in 0.0f64..=64.0,
        meter_a in arb_meter(),
        meter_b in arb_meter(),
        bars_a in 1u32..=32,
        bars_b in 1u32..=32,
    ) {
        let start = TimePoint::new(start);
        let grid_a = GridConfig::new(start, meter_a).bars(bars_a).build();
        let handoff = *grid_a.measures().last().unwrap();
        let grid_b = GridConfig::new(handoff, meter_b).bars(bars_b).build();

        prop_assert!((grid_b.measures().first().unwrap().as_beats() - handoff.as_beats()).abs() < 1e-9);
        prop_assert!((grid_b.beats().first().unwrap().as_beats() - handoff.as_beats()).abs() < 1e-9);
        let first_bar = grid_b.measures()[1] - grid_b.measures()[0];
        prop_assert!((first_bar.as_beats() - meter_b.bar_span().as_beats()).abs() <= 1e-9 * (1.0 + meter_b.bar_span().as_beats()));
    }
}
