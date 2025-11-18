//! N-gram and Markov chain models for musical sequence prediction.
//!
//! This module provides statistical models to forecast likely chord or melody
//! continuations based on observed sequences. Both models support tunable
//! lookahead depth and can generate multiple candidate predictions with
//! associated probabilities.

use std::collections::HashMap;

/// A generic trait for sequence-based prediction models.
///
/// Implementations should maintain transition probabilities learned from
/// training sequences and support generation of likely continuations.
pub trait TransitionModel<T: Clone + Eq + std::hash::Hash> {
    /// Train the model on a sequence of tokens.
    ///
    /// For an n-gram model with order `n`, this builds transition counts
    /// from subsequences of length `n` to their successors.
    fn train(&mut self, sequence: &[T]);

    /// Predict the next `count` tokens given a context.
    ///
    /// Returns a list of predictions, each containing a candidate token and
    /// its probability/confidence score. If the context has never been seen,
    /// returns an empty vector.
    ///
    /// # Arguments
    /// - `context`: Recent tokens used to query the model (length matches model order)
    /// - `count`: Maximum number of predictions to return
    fn predict(&self, context: &[T], count: usize) -> Vec<Prediction<T>>;

    /// Clear all learned transition data.
    fn reset(&mut self);
}

/// A prediction containing a candidate token and its associated probability.
#[derive(Debug, Clone, PartialEq)]
pub struct Prediction<T> {
    /// The predicted token.
    pub token: T,
    /// Probability or confidence score in [0.0, 1.0].
    pub probability: f32,
}

/// An n-gram model for melody prediction based on pitch class transitions.
///
/// This model tracks sequences of pitch classes (0..12) and learns transition
/// probabilities. It can be used to extrapolate melodic continuations.
#[derive(Debug, Clone)]
pub struct MelodyTransitionModel {
    /// Model order (n-gram size minus one). For example, order=1 is bigrams.
    order: usize,
    /// Transition counts: (context, next_token) -> count
    transitions: HashMap<Vec<u8>, HashMap<u8, usize>>,
    /// Total observations for each context (for normalization)
    context_totals: HashMap<Vec<u8>, usize>,
}

impl MelodyTransitionModel {
    /// Create a new melody transition model with the specified order.
    ///
    /// # Arguments
    /// - `order`: N-gram order (1 = bigram, 2 = trigram, etc.)
    ///
    /// # Examples
    /// ```
    /// use music_analysis::MelodyTransitionModel;
    ///
    /// let model = MelodyTransitionModel::new(2); // trigram model
    /// ```
    #[must_use]
    pub fn new(order: usize) -> Self {
        Self {
            order: order.max(1),
            transitions: HashMap::new(),
            context_totals: HashMap::new(),
        }
    }

    /// Get the model's n-gram order.
    #[must_use]
    pub fn order(&self) -> usize {
        self.order
    }
}

impl TransitionModel<u8> for MelodyTransitionModel {
    fn train(&mut self, sequence: &[u8]) {
        if sequence.len() <= self.order {
            return;
        }

        for window in sequence.windows(self.order + 1) {
            let context = window[..self.order].to_vec();
            let next = window[self.order];

            *self
                .transitions
                .entry(context.clone())
                .or_default()
                .entry(next)
                .or_insert(0) += 1;
            *self.context_totals.entry(context).or_insert(0) += 1;
        }
    }

    fn predict(&self, context: &[u8], count: usize) -> Vec<Prediction<u8>> {
        if context.len() != self.order {
            return Vec::new();
        }

        let context_vec = context.to_vec();
        let Some(successors) = self.transitions.get(&context_vec) else {
            return Vec::new();
        };

        let total = self.context_totals.get(&context_vec).copied().unwrap_or(0);
        if total == 0 {
            return Vec::new();
        }

        let mut predictions: Vec<_> = successors
            .iter()
            .map(|(token, count)| Prediction {
                token: *token,
                probability: *count as f32 / total as f32,
            })
            .collect();

        predictions.sort_by(|a, b| {
            b.probability
                .partial_cmp(&a.probability)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        predictions.truncate(count);
        predictions
    }

    fn reset(&mut self) {
        self.transitions.clear();
        self.context_totals.clear();
    }
}

/// An n-gram model for chord progression prediction.
///
/// This model tracks sequences of chord symbols (represented as strings) and
/// learns transition probabilities useful for harmonic extrapolation.
#[derive(Debug, Clone)]
pub struct ChordTransitionModel {
    /// Model order (n-gram size minus one).
    order: usize,
    /// Transition counts: (context, next_chord) -> count
    transitions: HashMap<Vec<String>, HashMap<String, usize>>,
    /// Total observations for each context
    context_totals: HashMap<Vec<String>, usize>,
}

impl ChordTransitionModel {
    /// Create a new chord transition model with the specified order.
    ///
    /// # Arguments
    /// - `order`: N-gram order (1 = bigram, 2 = trigram, etc.)
    ///
    /// # Examples
    /// ```
    /// use music_analysis::ChordTransitionModel;
    ///
    /// let model = ChordTransitionModel::new(1); // bigram model
    /// ```
    #[must_use]
    pub fn new(order: usize) -> Self {
        Self {
            order: order.max(1),
            transitions: HashMap::new(),
            context_totals: HashMap::new(),
        }
    }

    /// Get the model's n-gram order.
    #[must_use]
    pub fn order(&self) -> usize {
        self.order
    }

    /// Train from a slice of Roman numerals or chord symbols.
    pub fn train_from_progression(&mut self, progression: &[impl AsRef<str>]) {
        let tokens: Vec<String> = progression.iter().map(|s| s.as_ref().to_string()).collect();
        self.train(&tokens);
    }

    /// Predict next chords given a context of recent chord symbols.
    pub fn predict_from_context(
        &self,
        context: &[impl AsRef<str>],
        count: usize,
    ) -> Vec<Prediction<String>> {
        let ctx: Vec<String> = context.iter().map(|s| s.as_ref().to_string()).collect();
        self.predict(&ctx, count)
    }
}

impl TransitionModel<String> for ChordTransitionModel {
    fn train(&mut self, sequence: &[String]) {
        if sequence.len() <= self.order {
            return;
        }

        for window in sequence.windows(self.order + 1) {
            let context = window[..self.order].to_vec();
            let next = window[self.order].clone();

            *self
                .transitions
                .entry(context.clone())
                .or_default()
                .entry(next)
                .or_insert(0) += 1;
            *self.context_totals.entry(context).or_insert(0) += 1;
        }
    }

    fn predict(&self, context: &[String], count: usize) -> Vec<Prediction<String>> {
        if context.len() != self.order {
            return Vec::new();
        }

        let context_vec = context.to_vec();
        let Some(successors) = self.transitions.get(&context_vec) else {
            return Vec::new();
        };

        let total = self.context_totals.get(&context_vec).copied().unwrap_or(0);
        if total == 0 {
            return Vec::new();
        }

        let mut predictions: Vec<_> = successors
            .iter()
            .map(|(token, count)| Prediction {
                token: token.clone(),
                probability: *count as f32 / total as f32,
            })
            .collect();

        predictions.sort_by(|a, b| {
            b.probability
                .partial_cmp(&a.probability)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        predictions.truncate(count);
        predictions
    }

    fn reset(&mut self) {
        self.transitions.clear();
        self.context_totals.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn melody_model_bigram() {
        let mut model = MelodyTransitionModel::new(1);
        // Train on C-D-E-D-C pattern (pitch classes 0,2,4,2,0)
        model.train(&[0, 2, 4, 2, 0]);

        // After seeing pitch class 2, we should predict both 4 and 0
        let predictions = model.predict(&[2], 2);
        assert_eq!(predictions.len(), 2);

        // Both should have equal probability (each appears once after 2)
        assert!((predictions[0].probability - 0.5).abs() < 0.01);
        assert!((predictions[1].probability - 0.5).abs() < 0.01);
    }

    #[test]
    fn chord_model_bigram() {
        let mut model = ChordTransitionModel::new(1);
        // Train on I-IV-V-I and I-vi-ii-V-I progressions
        model.train_from_progression(&["I", "IV", "V", "I"]);
        model.train_from_progression(&["I", "vi", "ii", "V", "I"]);

        // After seeing "V", the model should predict "I" (appears twice)
        let predictions = model.predict_from_context(&["V"], 1);
        assert_eq!(predictions.len(), 1);
        assert_eq!(predictions[0].token, "I");
        assert!((predictions[0].probability - 1.0).abs() < 0.01);
    }

    #[test]
    fn empty_context_returns_empty() {
        let model = MelodyTransitionModel::new(2);
        let predictions = model.predict(&[0, 2], 5);
        assert!(predictions.is_empty());
    }

    #[test]
    fn reset_clears_model() {
        let mut model = ChordTransitionModel::new(1);
        model.train_from_progression(&["I", "V", "I"]);
        assert!(!model.predict_from_context(&["V"], 1).is_empty());

        model.reset();
        assert!(model.predict_from_context(&["V"], 1).is_empty());
    }
}
