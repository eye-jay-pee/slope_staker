use super::{SlopeStake, Station};
use std::collections::HashMap;

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
