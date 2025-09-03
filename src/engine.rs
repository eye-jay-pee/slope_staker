pub mod station {
    #[derive(Default)]
    pub struct Station {
        value: f32, // TODO implement as fixed-point value
    }
    impl std::fmt::Display for Station {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}+{:02.}",
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
        pub fn set_station_num(&mut self, new_val: i32) -> () {
            self.value = self.get_station_plus() + 100.0 * new_val as f32;
        }
        pub fn set_station_plus(&mut self, new_val: f32) -> () {
            self.value = self.get_station_num() as f32 + (new_val % 100.0)
        }
    }

    pub mod ui {
        use super::*;
        use eframe::egui::{Response, Ui, Widget};

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
                let inner = ui.horizontal(|ui| {
                    ui.label(format!("STA {}", self.sta));
                    let mut tmp = 5;
                    let _ = ui
                        .add(eframe::egui::DragValue::new(&mut tmp).speed(1.0));
                });
                inner.response
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
