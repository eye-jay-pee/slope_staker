#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Station {
    value: f32,
}
impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}+{:05.2}",
            self.get_station_num(),
            self.get_station_plus().abs()
        )
    }
}
impl Station {
    pub fn _new(station_number: i32, station_plus: f32) -> Self {
        Station {
            value: (station_number * 100) as f32 + station_plus,
        }
    }
    pub fn get_station_num(&self) -> i32 {
        (self.value / 100.0).floor() as i32
    }
    pub fn get_station_plus(&self) -> f32 {
        self.value % 100.0
    }
    pub fn _set_station_num(&mut self, new_val: i32) -> () {
        self.value = self.get_station_plus() + 100.0 * new_val as f32;
    }
    pub fn _set_station_plus(&mut self, new_val: f32) -> () {
        self.value = self.get_station_num() as f32 + (new_val % 100.0)
    }
    fn get_set_raw_sta(&mut self, new_val: Option<f64>) -> f64 {
        match new_val {
            Some(v) => self.value = v as f32,
            None => {}
        };
        self.value as f64
    }
    pub fn from_f64(v: f64) -> Self {
        Station { value: v as f32 }
    }
}

pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct StationEditor<'a>(&'a mut Station);
    impl<'a> Widget for StationEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            use eframe::egui::DragValue;

            let dv = DragValue::from_get_set(|v| self.0.get_set_raw_sta(v))
                .custom_formatter(|val, _| Station::from_f64(val).to_string());

            ui.horizontal(|ui| {
                ui.label("Station:");
                ui.add(dv)
            })
            .response
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_formatter_zero() {
        let sta = Station::default();
        assert_eq!(sta.to_string(), "0+00.00");
    }
    //#[test]
    //fn test_formatter_arbitrary() {
    //    let sta = Station::new(1234567.89);
    //    assert_eq!(sta.to_string(), "12345+67.89");
    //}
}
