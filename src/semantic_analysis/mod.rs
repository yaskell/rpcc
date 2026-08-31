mod identifier_resolution;
pub mod loop_labeling;
mod type_checking;

pub use identifier_resolution::resolve_identifiers;
pub use loop_labeling::label_loops;
pub use type_checking::check_types;
