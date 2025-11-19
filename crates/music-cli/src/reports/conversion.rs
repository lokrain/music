//! MIDI analysis, pitch/frequency conversion, and temperament remapping reports.

use std::fmt::Write;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiAnalysisReport {
    pub file: String,
    pub size_bytes: u64,
    pub header_format: Option<u16>,
    pub declared_tracks: Option<u16>,
    pub detected_tracks: usize,
    pub ticks_per_quarter: Option<u16>,
    pub key_hint: Option<String>,
    pub is_standard_midi: bool,
}

impl MidiAnalysisReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "MIDI file: {file} ({bytes} bytes).",
            file = self.file,
            bytes = self.size_bytes
        );
        let status = if self.is_standard_midi {
            "Standard MIDI header detected"
        } else {
            "Unknown/invalid MIDI header"
        };
        let _ = writeln!(&mut out, "{status}.");
        if let Some(format) = self.header_format {
            let _ = writeln!(
                &mut out,
                "Declared format: {format}, tracks: {declared:?}, detected chunks: {detected}.",
                declared = self.declared_tracks,
                detected = self.detected_tracks
            );
        } else {
            let _ = writeln!(
                &mut out,
                "Detected track chunks: {detected}.",
                detected = self.detected_tracks
            );
        }
        if let Some(ticks) = self.ticks_per_quarter {
            let _ = writeln!(&mut out, "Ticks per quarter note: {ticks}.", ticks = ticks);
        }
        if let Some(key) = &self.key_hint {
            let _ = writeln!(&mut out, "Context key hint: {key}.");
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PitchIndexToFrequency {
    pub system: String,
    pub index: i32,
    pub frequency_hz: f32,
    pub label: Option<String>,
}

impl PitchIndexToFrequency {
    pub fn render_text(&self) -> String {
        let label = self.label.as_deref().unwrap_or("(unnamed)");
        format!(
            "Index {index} in {system} ⇒ {freq:.3} Hz {label}",
            index = self.index,
            system = self.system,
            freq = self.frequency_hz,
            label = label
        )
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FrequencyToIndexReport {
    pub system: String,
    pub input_frequency_hz: f32,
    pub resolved_index: i32,
    pub resolved_frequency_hz: f32,
    pub cents_error: f32,
    pub search_center: i32,
    pub search_span: i32,
    pub label: Option<String>,
}

impl FrequencyToIndexReport {
    pub fn render_text(&self) -> String {
        let label = self.label.as_deref().unwrap_or("(unnamed)");
        format!(
            "{freq:.3} Hz ≈ index {index} in {system} ({label}) — Δ {error:+.2} cents [search center {center}, span {span}]",
            freq = self.input_frequency_hz,
            index = self.resolved_index,
            system = self.system,
            label = label,
            error = self.cents_error,
            center = self.search_center,
            span = self.search_span
        )
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemperamentRemapReport {
    pub from_system: String,
    pub to_system: String,
    pub mapping_count: usize,
    pub search_span: i32,
    pub mappings: Vec<TemperamentMappingRow>,
}

impl TemperamentRemapReport {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "Remapped {count} pitch(es) from {from} → {to} (search span ±{span}).",
            count = self.mapping_count,
            from = self.from_system,
            to = self.to_system,
            span = self.search_span
        );
        let _ = writeln!(
            &mut out,
            "source_index,source_label,source_hz,target_index,target_label,target_hz,cents_delta"
        );
        for mapping in &self.mappings {
            let _ = writeln!(
                &mut out,
                "{src_idx},{src_label},{src_hz:.3},{dst_idx},{dst_label},{dst_hz:.3},{delta:+.2}",
                src_idx = mapping.source_index,
                src_label = mapping.source_label.as_deref().unwrap_or(""),
                src_hz = mapping.source_frequency_hz,
                dst_idx = mapping.target_index,
                dst_label = mapping.target_label.as_deref().unwrap_or(""),
                dst_hz = mapping.target_frequency_hz,
                delta = mapping.cents_delta
            );
        }
        out
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemperamentMappingRow {
    pub source_index: i32,
    pub source_label: Option<String>,
    pub source_frequency_hz: f32,
    pub target_index: i32,
    pub target_label: Option<String>,
    pub target_frequency_hz: f32,
    pub cents_delta: f32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiCsvConversionReport {
    pub direction: MidiConversionDirection,
    pub source: String,
    pub destination: Option<String>,
    pub note_count: usize,
    pub emitted_rows: usize,
    pub truncated: bool,
    pub ticks_per_quarter: Option<u16>,
    pub rows: Vec<MidiCsvRow>,
}

impl MidiCsvConversionReport {
    pub fn render_text(&self) -> String {
        match self.direction {
            MidiConversionDirection::MidiToCsv => self.render_as_csv(),
            MidiConversionDirection::CsvToMidi => self.render_csv_to_midi_summary(),
        }
    }

    fn render_as_csv(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            &mut out,
            "track,channel,note,start_tick,end_tick,duration_tick,velocity"
        );
        for row in &self.rows {
            let _ = writeln!(
                &mut out,
                "{track},{channel},{note},{start},{end},{dur},{vel}",
                track = row.track,
                channel = row.channel,
                note = row.note,
                start = row.start_tick,
                end = row.end_tick,
                dur = row.duration_tick,
                vel = row.velocity
            );
        }
        if self.truncated {
            let remaining = self.note_count.saturating_sub(self.rows.len());
            let _ = writeln!(
                &mut out,
                "# truncated {remaining} additional row(s); re-run with --max-rows=<larger> to emit all"
            );
        }
        out
    }

    fn render_csv_to_midi_summary(&self) -> String {
        let mut out = String::new();
        let destination = self.destination.as_deref().unwrap_or("(not written)");
        let ticks = self
            .ticks_per_quarter
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".into());
        let _ = writeln!(
            &mut out,
            "Wrote {count} note(s) to {dest} (PPQ {ppq}).",
            count = self.note_count,
            dest = destination,
            ppq = ticks
        );
        let preview = self.rows.len().min(16);
        if preview > 0 {
            let _ = writeln!(
                &mut out,
                "Preview of first {preview} row(s): track,channel,note,start_tick,end_tick,duration_tick,velocity",
                preview = preview
            );
            for row in self.rows.iter().take(preview) {
                let _ = writeln!(
                    &mut out,
                    "{track},{channel},{note},{start},{end},{dur},{vel}",
                    track = row.track,
                    channel = row.channel,
                    note = row.note,
                    start = row.start_tick,
                    end = row.end_tick,
                    dur = row.duration_tick,
                    vel = row.velocity
                );
            }
        }
        out
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MidiConversionDirection {
    MidiToCsv,
    CsvToMidi,
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MidiCsvRow {
    pub track: u16,
    pub channel: u8,
    pub note: u8,
    pub start_tick: u32,
    pub end_tick: u32,
    pub duration_tick: u32,
    pub velocity: u8,
}
