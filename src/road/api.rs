use super::{SlopeStake, Station};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Road {
    _left: HashMap<Station, SlopeStake>,
    _right: HashMap<Station, SlopeStake>,
}
impl Default for Road {
    fn default() -> Self {
        Self {
            _left: HashMap::new(),
            _right: HashMap::new(),
        }
    }
}
