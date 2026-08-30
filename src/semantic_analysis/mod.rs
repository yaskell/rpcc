mod identifier_resolution;
pub mod loop_labeling;

pub use identifier_resolution::resolve_identifiers;
pub use loop_labeling::label_loops;
