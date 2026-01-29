use std::collections::HashMap;
use serde_yaml::Value;

/// A Membrane project is deliberately schema-less.
/// Keys are language artifacts, not fields.
pub type Project = HashMap<String, Value>;

