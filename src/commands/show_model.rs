use crate::core::Project;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ShowContext {
    pub brane_root: PathBuf,
    pub brane_id: String,
    pub sort_key: Option<String>,
    pub projects: Vec<(String, Project)>,
}
