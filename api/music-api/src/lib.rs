//! Public API surface for higher-level HTTP/CLI layers.

#![deny(missing_docs)]

/// Data transfer objects exposed by the API layer.
pub mod models;

pub use models::plan::{
    BarSummary, CadenceSummary, ExplainModeDto, KeySpecification, ModeDto, PhraseSummary,
    PlanRequest, PlanResponse, StateSnapshot, StyleOverrides, StyleSpecification,
    TemplateDescriptor, TemplateFormatDto, TemplateSelector, TemplateSourceDescriptor,
};
