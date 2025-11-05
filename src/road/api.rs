use super::{CrossSection, Station};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Deserialize, Serialize)]
pub struct Road {
    pub cross_sections: HashMap<Station, CrossSection>,
}

impl Road {
    pub fn _specified_stations(&self) -> Vec<Station> {
        self.cross_sections.keys().cloned().collect()
    }
    pub fn insert_new(&mut self, station: Station) {
        let _ = self.cross_sections.insert(station, CrossSection::default());
    }
}
