use music_core::{AbstractPitch, PitchSystemId};

#[test]
fn arithmetic_helpers_and_components_match_expectations() {
    let id = PitchSystemId::from("12tet");
    let pitch = AbstractPitch::new(60, id.clone());

    assert_eq!(pitch.transpose(12).components(), (72, &id));
    assert_eq!((pitch.clone() + 1).components().0, 61);
    assert_eq!((pitch.clone() - 1).components().0, 59);

    let mut mutable = pitch.clone();
    mutable += 2;
    assert_eq!(mutable.components().0, 62);
    mutable -= 4;
    assert_eq!(mutable.components().0, 58);
}

#[test]
fn tuple_constructors_round_trip() {
    let id = PitchSystemId::from("edo");
    let from_owned = AbstractPitch::from((45, id.clone()));
    assert_eq!(from_owned.components(), (45, &id));

    let from_ref = AbstractPitch::from((30, &id));
    assert_eq!(from_ref.components(), (30, &id));
}

#[test]
fn display_formats_index_and_system() {
    let id = PitchSystemId::from("just");
    let pitch = AbstractPitch::new(10, id.clone());
    assert_eq!(format!("{pitch}"), format!("10@{}", id));
}
