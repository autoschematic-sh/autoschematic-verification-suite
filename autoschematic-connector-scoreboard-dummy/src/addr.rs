use std::path::{Path, PathBuf};

use autoschematic_core::{connector::ResourceAddress, error_util::invalid_addr_path};

impl ResourceAddress for ScoreboardAddress {
    fn to_path_buf(&self) -> std::path::PathBuf {
        match self {
            ScoreboardAddress::Resource {} => PathBuf::from("scoreboard/resource.ron"),
            ScoreboardAddress::Bundle {} => PathBuf::from("scoreboard/bundle.ron"),
        }
    }

    fn from_path(path: &std::path::Path) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        if path == PathBuf::from("scoreboard/resource.ron") {
            Ok(ScoreboardAddress::Resource {})
        } else if path == PathBuf::from("scoreboard/bundle.ron") {
            Ok(ScoreboardAddress::Bundle {})
        } else {
            Err(invalid_addr_path(path))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScoreboardAddress {
    Resource {},
    Bundle {},
}
