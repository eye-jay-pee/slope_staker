use super::{CrossSection, Station};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Road(pub HashMap<Station, CrossSection>);

impl Default for Road {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
