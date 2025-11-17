use music_core::{Pitch, PitchSystemId};

#[test]
fn serde_round_trip_retains_data() {
    let pitch = Pitch::abstract_pitch(42, PitchSystemId::from("edo"));
    let json = serde_json::to_string(&pitch).expect("serialize");
    let restored: Pitch = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored, pitch);
}
