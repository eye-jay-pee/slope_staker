pub mod station {
    #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Station {
        value: f32, // TODO implement as fixed-point value
    }
    impl std::fmt::Display for Station {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}+{:05.2}",
                self.get_station_num(),
                self.get_station_plus()
            )
        }
    }
    impl Station {
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
    }

    pub mod ui {
        use super::*;
        use eframe::egui::{Response, Ui, Widget};

        impl Station {
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

        pub struct StationEditor<'a> {
            sta: &'a mut Station,
        }
        impl<'a> StationEditor<'a> {
            pub fn new(sta: &'a mut Station) -> Self {
                Self { sta: sta }
            }
        }
        impl<'a> Widget for StationEditor<'a> {
            fn ui(self, ui: &mut Ui) -> Response {
                ui.add(
                    eframe::egui::DragValue::from_get_set(|v| {
                        self.sta.get_set_raw_sta(v)
                    })
                    .custom_formatter(|val, _| {
                        Station::from_f64(val).to_string()
                    })
                    .speed(1.0),
                )
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
        #[test]
        fn test_formatter_arbitrary() {
            let sta = Station::new(1234567.89);
            assert_eq!(sta.to_string(), "12345+67.89");
        }
    }
}
