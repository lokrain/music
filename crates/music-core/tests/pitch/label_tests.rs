use music_core::PitchLabel;

#[test]
fn symbolic_label_reports_name() {
    let label = PitchLabel::Named("A4".into());
    assert!(label.is_symbolic());
    assert_eq!(label.to_string_lossy(), "A4");
    assert_eq!(label.as_frequency(), None);
}

#[test]
fn frequency_label_formats_and_exposes_hz() {
    let label = PitchLabel::Frequency(261.6256);
    assert!(!label.is_symbolic());
    assert_eq!(label.as_frequency(), Some(261.6256));
    assert_eq!(label.to_string(), "261.626 Hz");
}
