use anyhow::{Result, anyhow, bail};

use crate::{
    cli::{InterpolateCommand, InterpolateTempoArgs, InterpolateVelocityArgs, TempoUnit},
    format::OutputFormat,
    responses::{
        InterpolatedEnvelopeReport, InterpolatedPoint, InterpolationContext, VelocityEnvelopeReport,
    },
};

pub fn handle_interpolate(
    _engine: &music_engine::MusicEngine,
    format: OutputFormat,
    command: InterpolateCommand,
) -> Result<()> {
    match command {
        InterpolateCommand::Tempo(args) => interpolate_tempo(format, args),
        InterpolateCommand::Velocity(args) => interpolate_velocity(format, args),
    }
}

fn interpolate_tempo(format: OutputFormat, args: InterpolateTempoArgs) -> Result<()> {
    let anchors = parse_anchors(&args.common.points, "tempo")?;
    if anchors.len() < 2 {
        bail!("provide at least two anchor points via --points");
    }
    let samples: Vec<InterpolatedPoint> =
        build_envelope(&anchors, args.common.samples, args.common.curve);
    let context = InterpolationContext {
        curve: format!("{:?}", args.common.curve),
        samples: samples.len(),
    };
    let report = InterpolatedEnvelopeReport {
        context,
        unit: match args.unit {
            TempoUnit::Bpm => "bpm".into(),
            TempoUnit::Multiplier => "multiplier".into(),
        },
        anchors,
        samples,
    };
    format.emit(&report, InterpolatedEnvelopeReport::render_text)
}

fn interpolate_velocity(format: OutputFormat, args: InterpolateVelocityArgs) -> Result<()> {
    let anchors = parse_anchors(&args.common.points, "velocity")?;
    if anchors.len() < 2 {
        bail!("provide at least two anchor points via --points");
    }
    let samples: Vec<InterpolatedPoint> =
        build_envelope(&anchors, args.common.samples, args.common.curve)
            .into_iter()
            .map(|mut point| {
                point.value = point
                    .value
                    .clamp(args.min_value as f32, args.max_value as f32);
                point
            })
            .collect();
    let context = InterpolationContext {
        curve: format!("{:?}", args.common.curve),
        samples: samples.len(),
    };
    let report = VelocityEnvelopeReport {
        context,
        anchors,
        samples,
        min_value: args.min_value,
        max_value: args.max_value,
    };
    format.emit(&report, VelocityEnvelopeReport::render_text)
}

fn parse_anchors(entries: &[String], label: &str) -> Result<Vec<InterpolatedPoint>> {
    let mut points = Vec::with_capacity(entries.len());
    for entry in entries {
        let parts: Vec<&str> = entry.split(':').collect();
        if parts.len() != 2 {
            bail!("anchors must use time:value format (got '{entry}' for {label})");
        }
        let time = parts[0]
            .trim()
            .parse::<f32>()
            .map_err(|_| anyhow!("invalid time '{entry}'"))?;
        let value = parts[1]
            .trim()
            .parse::<f32>()
            .map_err(|_| anyhow!("invalid value '{entry}'"))?;
        points.push(InterpolatedPoint { time, value });
    }
    points.sort_by(|a, b| {
        a.time
            .partial_cmp(&b.time)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(points)
}

fn build_envelope(
    anchors: &[InterpolatedPoint],
    samples: usize,
    curve: crate::cli::InterpolationCurve,
) -> Vec<InterpolatedPoint> {
    if anchors.len() < 2 || samples == 0 {
        return anchors.to_vec();
    }
    let mut output = Vec::with_capacity(samples * anchors.len());
    for window in anchors.windows(2) {
        let start = window[0].clone();
        let end = window[1].clone();
        for step in 0..samples {
            let t = step as f32 / samples as f32;
            let eased = apply_curve(t, curve);
            let time = start.time + (end.time - start.time) * eased;
            let value = start.value + (end.value - start.value) * eased;
            output.push(InterpolatedPoint { time, value });
        }
    }
    if let Some(last) = anchors.last() {
        output.push(last.clone());
    }
    output
}

fn apply_curve(t: f32, curve: crate::cli::InterpolationCurve) -> f32 {
    match curve {
        crate::cli::InterpolationCurve::Linear => t,
        crate::cli::InterpolationCurve::EaseIn => t * t,
        crate::cli::InterpolationCurve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
        crate::cli::InterpolationCurve::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
    }
}
