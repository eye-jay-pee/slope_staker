use super::{SlopeStake, Station};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct _Road {
    _left: HashMap<Station, SlopeStake>,
    _right: HashMap<Station, SlopeStake>,
}
impl Default for _Road {
    fn default() -> Self {
        Self {
            _left: HashMap::new(),
            _right: HashMap::new(),
        }
    }
}
