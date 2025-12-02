use std::path::{PathBuf};

use autoschematic_core::{connector::ResourceAddress, error_util::invalid_addr_path};

impl ResourceAddress for ScoreboardAddress {
    fn to_path_buf(&self) -> std::path::PathBuf {
        match self {
            ScoreboardAddress::Resource {} => PathBuf::from("scoreboard/resource.ron"),
            ScoreboardAddress::Bundle {} => PathBuf::from("scoreboard/bundle.ron"),
            ScoreboardAddress::Task(ScoreboardTaskType::CountDown) => {
                PathBuf::from("scoreboard/task/count_down.ron")
            }
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
        } else if path == PathBuf::from("scoreboard/task/count_down.ron") {
            Ok(ScoreboardAddress::Task(ScoreboardTaskType::CountDown))
        } else {
            Err(invalid_addr_path(path))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScoreboardTaskType {
    CountDown,
}

#[derive(Debug, Clone)]
pub enum ScoreboardAddress {
    Resource,
    Bundle,
    Task(ScoreboardTaskType),
}
