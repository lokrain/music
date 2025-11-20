use music_time::{Tempo, TimeSpan, meter::Meter};
use proptest::prelude::*;

fn arb_meter() -> impl Strategy<Value = Meter> {
    (1u8..=12, prop_oneof![Just(2u8), Just(4u8), Just(8u8), Just(16u8)])
        .prop_map(|(numerator, denominator)| Meter::new(numerator, denominator))
}

proptest! {
    #[test]
    fn tempo_span_seconds_roundtrip(
        bpm in 10.0f64..=360.0,
        beats in 0.0f64..=512.0,
    ) {
        let tempo = Tempo::new(bpm);
        let span = TimeSpan::new(beats);
        let seconds = tempo.seconds_for_span(span);
        let roundtrip = tempo.span_for_seconds(seconds);
        prop_assert!((roundtrip.as_beats() - span.as_beats()).abs() <= 1e-9 * (1.0 + span.as_beats().abs()));
    }

    #[test]
    fn tempo_seconds_per_bar_matches_meter(
        bpm in 10.0f64..=480.0,
        meter in arb_meter(),
    ) {
        let tempo = Tempo::new(bpm);
        let seconds = tempo.seconds_per_bar(meter);
        let beats_per_bar = meter.bar_span().as_beats();
        prop_assert!((seconds - beats_per_bar * tempo.seconds_per_beat()).abs() <= 1e-9 * (1.0 + seconds.abs()));
    }
}
