use music_core::{
    chord::{Accidental, ChordQuality, Inversion, NoteLetter, ParseChordError, parse_chord_symbol},
    registry::TuningRegistry,
    system::PitchSystemId,
    systems::TwelveTET,
};

fn registry() -> TuningRegistry {
    TuningRegistry::new().with_system(PitchSystemId::from("12tet"), TwelveTET::a4_440())
}

#[test]
fn extended_qualities_define_correct_tone_counts() {
    assert_eq!(ChordQuality::Add9.tone_count(), 4);
    assert_eq!(ChordQuality::Dominant9.tone_count(), 5);
    assert_eq!(ChordQuality::Major9.tone_count(), 5);
    assert_eq!(ChordQuality::Minor9.tone_count(), 5);
    assert_eq!(ChordQuality::Dominant11.tone_count(), 5);
    assert_eq!(ChordQuality::Major11.tone_count(), 5);
    assert_eq!(ChordQuality::Minor11.tone_count(), 5);
    assert_eq!(ChordQuality::Dominant13.tone_count(), 5);
    assert_eq!(ChordQuality::Major13.tone_count(), 5);
    assert_eq!(ChordQuality::Minor13.tone_count(), 5);
}

#[test]
fn extended_qualities_provide_correct_semitone_offsets() {
    assert_eq!(ChordQuality::Add9.semitone_offsets(), &[0, 4, 7, 14]);
    assert_eq!(
        ChordQuality::Dominant9.semitone_offsets(),
        &[0, 4, 7, 10, 14]
    );
    assert_eq!(ChordQuality::Major9.semitone_offsets(), &[0, 4, 7, 11, 14]);
    assert_eq!(ChordQuality::Minor9.semitone_offsets(), &[0, 3, 7, 10, 14]);
    assert_eq!(
        ChordQuality::Dominant11.semitone_offsets(),
        &[0, 4, 7, 10, 17]
    );
    assert_eq!(ChordQuality::Major11.semitone_offsets(), &[0, 4, 7, 11, 17]);
    assert_eq!(ChordQuality::Minor11.semitone_offsets(), &[0, 3, 7, 10, 17]);
    assert_eq!(
        ChordQuality::Dominant13.semitone_offsets(),
        &[0, 4, 7, 10, 21]
    );
    assert_eq!(ChordQuality::Major13.semitone_offsets(), &[0, 4, 7, 11, 21]);
    assert_eq!(ChordQuality::Minor13.semitone_offsets(), &[0, 3, 7, 10, 21]);
}

#[test]
fn extended_chords_build_correctly() {
    let registry = registry();
    let system = PitchSystemId::from("12tet");
    let chord = ChordQuality::Dominant9
        .build_chord(60, system, &registry)
        .unwrap();
    assert_eq!(chord.tone_count(), 5);
    let tones = chord.tones(&registry).unwrap();
    assert_eq!(tones.len(), 5);
}

#[test]
fn inversion_from_bass_index() {
    assert_eq!(Inversion::from_bass_index(0), Inversion::Root);
    assert_eq!(Inversion::from_bass_index(1), Inversion::First);
    assert_eq!(Inversion::from_bass_index(2), Inversion::Second);
    assert_eq!(Inversion::from_bass_index(3), Inversion::Third);
    assert_eq!(Inversion::from_bass_index(4), Inversion::Higher(4));
}

#[test]
fn inversion_bass_index_round_trip() {
    let root = Inversion::Root;
    assert_eq!(Inversion::from_bass_index(root.bass_index()), root);
    let first = Inversion::First;
    assert_eq!(Inversion::from_bass_index(first.bass_index()), first);
    let second = Inversion::Second;
    assert_eq!(Inversion::from_bass_index(second.bass_index()), second);
    let third = Inversion::Third;
    assert_eq!(Inversion::from_bass_index(third.bass_index()), third);
}

#[test]
fn detect_inversion_for_root_position() {
    let registry = registry();
    let system = PitchSystemId::from("12tet");
    let chord = ChordQuality::MajorTriad
        .build_chord(60, system, &registry)
        .unwrap();
    let tones = chord.tones(&registry).unwrap(); // [C, E, G]
    let inversion = chord.detect_inversion(&tones, &registry).unwrap();
    assert_eq!(inversion, Some(Inversion::Root));
}

#[test]
fn detect_inversion_for_first_inversion() {
    let registry = registry();
    let system = PitchSystemId::from("12tet");
    let chord = ChordQuality::MajorTriad
        .build_chord(60, system, &registry)
        .unwrap();
    let tones = chord.tones(&registry).unwrap(); // [C, E, G]
    let voiced = vec![tones[1].clone(), tones[2].clone(), tones[0].clone()]; // [E, G, C]
    let inversion = chord.detect_inversion(&voiced, &registry).unwrap();
    assert_eq!(inversion, Some(Inversion::First));
}

#[test]
fn detect_inversion_for_second_inversion() {
    let registry = registry();
    let system = PitchSystemId::from("12tet");
    let chord = ChordQuality::MajorTriad
        .build_chord(60, system, &registry)
        .unwrap();
    let tones = chord.tones(&registry).unwrap(); // [C, E, G]
    let voiced = vec![tones[2].clone(), tones[0].clone(), tones[1].clone()]; // [G, C, E]
    let inversion = chord.detect_inversion(&voiced, &registry).unwrap();
    assert_eq!(inversion, Some(Inversion::Second));
}

#[test]
fn parse_simple_major_chord() {
    let symbol = parse_chord_symbol("C").unwrap();
    assert_eq!(symbol.root, NoteLetter::C);
    assert_eq!(symbol.accidental, Accidental::Natural);
    assert_eq!(symbol.quality, ChordQuality::MajorTriad);
    assert_eq!(symbol.root_semitone(), 0);
}

#[test]
fn parse_minor_chord_with_sharp() {
    let symbol = parse_chord_symbol("F#m").unwrap();
    assert_eq!(symbol.root, NoteLetter::F);
    assert_eq!(symbol.accidental, Accidental::Sharp);
    assert_eq!(symbol.quality, ChordQuality::MinorTriad);
    assert_eq!(symbol.root_semitone(), 6);
}

#[test]
fn parse_diminished_chord_with_flat() {
    let symbol = parse_chord_symbol("Bbdim").unwrap();
    assert_eq!(symbol.root, NoteLetter::B);
    assert_eq!(symbol.accidental, Accidental::Flat);
    assert_eq!(symbol.quality, ChordQuality::DiminishedTriad);
    assert_eq!(symbol.root_semitone(), 10);
}

#[test]
fn parse_dominant_seventh() {
    let symbol = parse_chord_symbol("G7").unwrap();
    assert_eq!(symbol.root, NoteLetter::G);
    assert_eq!(symbol.accidental, Accidental::Natural);
    assert_eq!(symbol.quality, ChordQuality::DominantSeventh);
}

#[test]
fn parse_major_seventh() {
    let symbol = parse_chord_symbol("Cmaj7").unwrap();
    assert_eq!(symbol.root, NoteLetter::C);
    assert_eq!(symbol.quality, ChordQuality::MajorSeventh);
}

#[test]
fn parse_minor_seventh() {
    let symbol = parse_chord_symbol("Am7").unwrap();
    assert_eq!(symbol.root, NoteLetter::A);
    assert_eq!(symbol.quality, ChordQuality::MinorSeventh);
}

#[test]
fn parse_half_diminished() {
    let symbol = parse_chord_symbol("Bm7b5").unwrap();
    assert_eq!(symbol.root, NoteLetter::B);
    assert_eq!(symbol.quality, ChordQuality::HalfDiminishedSeventh);
}

#[test]
fn parse_extended_chords() {
    let dom9 = parse_chord_symbol("D9").unwrap();
    assert_eq!(dom9.quality, ChordQuality::Dominant9);

    let maj9 = parse_chord_symbol("Emaj9").unwrap();
    assert_eq!(maj9.quality, ChordQuality::Major9);

    let min9 = parse_chord_symbol("Fm9").unwrap();
    assert_eq!(min9.quality, ChordQuality::Minor9);

    let add9 = parse_chord_symbol("Gadd9").unwrap();
    assert_eq!(add9.quality, ChordQuality::Add9);

    let dom11 = parse_chord_symbol("A11").unwrap();
    assert_eq!(dom11.quality, ChordQuality::Dominant11);

    let dom13 = parse_chord_symbol("B13").unwrap();
    assert_eq!(dom13.quality, ChordQuality::Dominant13);
}

#[test]
fn parse_suspended_chords() {
    let sus2 = parse_chord_symbol("Csus2").unwrap();
    assert_eq!(sus2.quality, ChordQuality::SuspendedSecond);

    let sus4 = parse_chord_symbol("Dsus4").unwrap();
    assert_eq!(sus4.quality, ChordQuality::SuspendedFourth);
}

#[test]
fn parse_augmented_chord() {
    let aug = parse_chord_symbol("Eaug").unwrap();
    assert_eq!(aug.quality, ChordQuality::AugmentedTriad);
}

#[test]
fn parse_empty_input_errors() {
    let err = parse_chord_symbol("").unwrap_err();
    assert_eq!(err, ParseChordError::EmptyInput);
}

#[test]
fn parse_invalid_root_errors() {
    let err = parse_chord_symbol("X7").unwrap_err();
    assert!(matches!(err, ParseChordError::InvalidRoot('X')));
}

#[test]
fn parse_unknown_quality_errors() {
    let err = parse_chord_symbol("Cfoo").unwrap_err();
    assert!(matches!(err, ParseChordError::UnknownQuality(_)));
}

#[test]
fn chord_symbol_display_round_trip() {
    let inputs = vec![
        "C", "F#m", "Bbdim", "G7", "Cmaj7", "Am7", "D9", "Emaj9", "Csus2", "Eaug",
    ];
    for input in inputs {
        let symbol = parse_chord_symbol(input).unwrap();
        let displayed = format!("{symbol}");
        // Parse the displayed version and verify it matches
        let reparsed = parse_chord_symbol(&displayed).unwrap();
        assert_eq!(symbol.root, reparsed.root);
        assert_eq!(symbol.quality, reparsed.quality);
    }
}
