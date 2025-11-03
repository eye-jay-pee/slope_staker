use super::SlopeStake;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CrossSection {
    pub left: SlopeStake,
    pub right: SlopeStake,
}
