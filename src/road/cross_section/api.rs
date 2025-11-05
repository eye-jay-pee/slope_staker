use super::SlopeStake;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct CrossSection {
    pub left: Option<SlopeStake>,
    pub right: Option<SlopeStake>,
}
