use crate::planner_util::{TemplateLocator, format_key_label, pitch_class_label};
use music_api::{
    BarSummary, CadenceSummary, ExplainModeDto, KeySpecification, ModeDto, PhraseSummary,
    PlanResponse, StateSnapshot, TemplateDescriptor, TemplateSourceDescriptor,
};
use music_score::planner::{ExplainMode, ExplainSummaries, PlannedSection, SectionTemplate};
use music_theory::{Key12, key::Mode};

pub fn print_text_report(
    template: &SectionTemplate,
    key: Key12,
    locator: &TemplateLocator,
    style_label: &str,
    explain_mode: ExplainMode,
    planned: &PlannedSection,
    summaries: &ExplainSummaries,
) {
    println!(
        "Template: {} (bars: {}, source: {})",
        template.metadata.id,
        template.bars,
        locator.describe()
    );
    println!("Key: {}", format_key_label(key));
    println!("Style: {} | Explain: {:?}", style_label, explain_mode);

    if planned.diagnostics.is_empty() {
        println!("Diagnostics: none");
    } else {
        println!("Diagnostics:");
        for diag in &planned.diagnostics {
            println!("  - {diag}");
        }
    }

    if summaries.bars.is_empty() {
        println!(
            "\nExplain summaries unavailable (use --explain brief|detailed for richer output)."
        );
    } else {
        println!("\nBars:");
        for bar in &summaries.bars {
            let phrase = bar.phrase.as_deref().unwrap_or("-");
            println!(
                "  Bar {:>2} [{:>6}] {:<12} -> {:<18} tension {:>4.2}→{:>4.2} cadence {}",
                bar.bar_index + 1,
                phrase,
                bar.function,
                bar.chord,
                bar.tension_target,
                bar.tension_actual,
                bar.cadence
            );
            if !bar.notes.is_empty() {
                println!("      notes: {}", bar.notes.join("; "));
            }
        }
    }

    if !summaries.phrases.is_empty() {
        println!("\nPhrases:");
        for phrase in &summaries.phrases {
            println!(
                "  {} (bars {}–{}): cadences {:?}; highlights {:?}",
                phrase.phrase,
                phrase.start_bar + 1,
                phrase.end_bar + 1,
                phrase.cadences,
                phrase.highlights
            );
        }
    }

    if !summaries.cadences.is_empty() {
        println!("\nCadences:");
        for cadence in &summaries.cadences {
            let phrase = cadence.phrase.as_deref().unwrap_or("-");
            let expected = cadence.expected_function.clone().unwrap_or_else(|| "?".into());
            println!(
                "  Bar {:>2} [{}] {:?} (expected {expected})",
                cadence.bar_index + 1,
                phrase,
                cadence.cadence
            );
        }
    }
}

pub fn build_json_report(
    template: &SectionTemplate,
    key: Key12,
    locator: &TemplateLocator,
    style_label: &str,
    explain_mode: ExplainMode,
    planned: &PlannedSection,
    summaries: &ExplainSummaries,
) -> PlanResponse {
    let phrases: Vec<_> = template.metadata.phrases.iter().map(|p| p.name.clone()).collect();
    let states = planned
        .states
        .iter()
        .map(|state| {
            let phrase = phrases.get(state.phrase_position.phrase_index as usize).cloned();
            StateSnapshot {
                bar: state.bar_index + 1,
                scale_degree: state.scale_degree,
                function: format!("{:?}", state.function.kind),
                chord: format!("{:?}", state.chord),
                tension: state.tension,
                cadence: format!("{:?}", state.cadence),
                phrase,
            }
        })
        .collect();

    let bars = summaries
        .bars
        .iter()
        .map(|bar| BarSummary {
            bar: bar.bar_index + 1,
            phrase: bar.phrase.clone(),
            function: bar.function.clone(),
            chord: bar.chord.clone(),
            tension_target: bar.tension_target,
            tension_actual: bar.tension_actual,
            reharm_risk: bar.reharm_risk,
            cadence: bar.cadence.clone(),
            notes: bar.notes.clone(),
        })
        .collect();

    let phrases_report = summaries
        .phrases
        .iter()
        .map(|phrase| PhraseSummary {
            phrase: phrase.phrase.clone(),
            start_bar: phrase.start_bar + 1,
            end_bar: phrase.end_bar + 1,
            cadences: phrase.cadences.clone(),
            highlights: phrase.highlights.clone(),
        })
        .collect();

    let cadences = summaries
        .cadences
        .iter()
        .map(|cadence| CadenceSummary {
            bar: cadence.bar_index + 1,
            phrase: cadence.phrase.clone(),
            cadence: cadence.cadence.clone(),
            expected_function: cadence.expected_function.clone(),
        })
        .collect();

    PlanResponse {
        template: TemplateDescriptor {
            id: template.metadata.id.clone(),
            version: template.metadata.version,
            bars: template.bars,
            source: TemplateSourceDescriptor {
                builtin: locator.builtin_id.clone(),
                registry_id: None,
                path: locator.file_path.clone(),
            },
        },
        key: KeySpecification {
            tonic: pitch_class_label(key.tonic).to_string(),
            mode: mode_dto(key.mode),
        },
        style: style_label.to_string(),
        explain_mode: explain_mode_dto(explain_mode),
        diagnostics: planned.diagnostics.clone(),
        states,
        bars,
        phrases: phrases_report,
        cadences,
    }
}

fn mode_dto(mode: Mode) -> ModeDto {
    match mode {
        Mode::Major => ModeDto::Major,
        Mode::Minor => ModeDto::Minor,
    }
}

fn explain_mode_dto(mode: ExplainMode) -> ExplainModeDto {
    match mode {
        ExplainMode::None => ExplainModeDto::None,
        ExplainMode::Brief => ExplainModeDto::Brief,
        ExplainMode::Detailed => ExplainModeDto::Detailed,
        ExplainMode::Debug => ExplainModeDto::Debug,
    }
}
