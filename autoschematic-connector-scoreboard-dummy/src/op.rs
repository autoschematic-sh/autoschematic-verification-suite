use autoschematic_core::{connector::ConnectorOp, util::RON};
use serde::{Deserialize, Serialize};

use crate::resource::ScoreboardState;


#[derive(Debug, Serialize, Deserialize)]
pub enum ScoreboardConnectorOp {
    SetState(ScoreboardState)
}

impl ConnectorOp for ScoreboardConnectorOp {
    fn to_string(&self) -> Result<String, anyhow::Error> {
        Ok(RON.to_string(self)?)
    }

    fn from_str(s: &str) -> Result<Self, anyhow::Error> where Self: Sized {
        Ok(RON.from_str(s)?)
    }
}