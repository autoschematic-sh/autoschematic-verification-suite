use autoschematic_core::{
    connector::Resource,
    util::{PrettyConfig, RON},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScoreboardState {
    pub random_int: i32
}

impl Resource for ScoreboardState {
    fn to_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(RON.to_string_pretty(self, PrettyConfig::new())?.into())
    }

    fn from_bytes(_addr: &impl autoschematic_core::connector::ResourceAddress, s: &[u8]) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let s = str::from_utf8(s)?;
        let state: ScoreboardState = RON.from_str(s)?;
        Ok(state)
    }
}
