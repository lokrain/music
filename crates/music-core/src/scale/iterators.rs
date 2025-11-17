use crate::{TuningRegistry, interval::Interval, pitch::Pitch};

use super::{Scale, ScaleDegreeError};

/// Lazily traverses the degrees of a [`Scale`], yielding both the accumulated interval and pitch.
///
/// Each iterator item contains the absolute degree index, the interval accumulated from the root,
/// and the resolved pitch for that degree.
#[derive(Clone)]
pub struct ScaleDegrees<'a> {
    pub(super) scale: &'a Scale,
    pub(super) registry: &'a TuningRegistry,
    pub(super) next_degree: usize,
    pub(super) current_pitch: Pitch,
    pub(super) current_interval: Option<Interval>,
}

impl<'a> ScaleDegrees<'a> {
    pub(super) fn new(scale: &'a Scale, registry: &'a TuningRegistry) -> Self {
        Self {
            scale,
            registry,
            next_degree: 0,
            current_pitch: scale.root.clone(),
            current_interval: None,
        }
    }
}

impl Iterator for ScaleDegrees<'_> {
    type Item = Result<(usize, Interval, Pitch), ScaleDegreeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_degree == 0 {
            self.next_degree += 1;
            return Some(Ok((0, Interval::identity(), self.scale.root.clone())));
        }

        if self.scale.pattern.steps.is_empty() {
            let degree = self.next_degree;
            self.next_degree += 1;
            return Some(Ok((degree, Interval::identity(), self.scale.root.clone())));
        }

        let len = self.scale.pattern.steps.len();
        let step = &self.scale.pattern.steps[(self.next_degree - 1) % len];
        let next_interval = match &self.current_interval {
            Some(interval) => match interval.compose(step) {
                Ok(value) => value,
                Err(err) => return Some(Err(err.into())),
            },
            None => step.clone(),
        };
        self.current_interval = Some(next_interval.clone());
        self.current_pitch = match step.apply_to(&self.current_pitch, self.registry) {
            Ok(value) => value,
            Err(err) => return Some(Err(err.into())),
        };

        let degree = self.next_degree;
        self.next_degree += 1;
        Some(Ok((degree, next_interval, self.current_pitch.clone())))
    }
}

impl core::iter::FusedIterator for ScaleDegrees<'_> {}

/// Iterator returned by [`Scale::degrees_up_to`], exposing an exact-length traversal.
#[derive(Clone)]
pub struct BoundedScaleDegrees<'a> {
    inner: ScaleDegrees<'a>,
    remaining: usize,
}

impl<'a> BoundedScaleDegrees<'a> {
    pub(super) fn new(
        scale: &'a Scale,
        highest_degree: usize,
        registry: &'a TuningRegistry,
    ) -> Self {
        Self {
            inner: ScaleDegrees::new(scale, registry),
            remaining: highest_degree.saturating_add(1),
        }
    }
}

impl Iterator for BoundedScaleDegrees<'_> {
    type Item = Result<(usize, Interval, Pitch), ScaleDegreeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let item = self.inner.next();
        if item.is_some() {
            self.remaining -= 1;
        } else {
            self.remaining = 0;
        }
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for BoundedScaleDegrees<'_> {}

impl core::iter::FusedIterator for BoundedScaleDegrees<'_> {}
