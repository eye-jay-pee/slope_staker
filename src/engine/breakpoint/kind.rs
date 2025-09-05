use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum BreakPointKind {
    #[default]
    _Centerline,
    _Ditch,
    _Unspecified,
    _Berm,
    _Limit,
    _Step,
    _DoesNotDaylight,
    _Shoulder,
}
impl std::fmt::Display for BreakPointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BreakPointKind::*;
        write!(
            f,
            "{}",
            match self {
                _Unspecified => "breakpoint",
                _Ditch => "ditch",
                _Berm => "berm",
                _DoesNotDaylight => "does not daylight",
                _Limit => "limit",
                _Shoulder => "shoulder",
                _Centerline => "crown",
                _Step => "step",
            }
        )
    }
}
pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct BreakPointKindSelector<'a>(&'a mut BreakPointKind);
    impl<'a> BreakPointKindSelector<'a> {
        pub fn new(kind: &'a mut BreakPointKind) -> Self {
            Self(kind)
        }
        fn check_hotkeys(&mut self, ui: &mut Ui) {
            use eframe::egui::Key;
            const MAPPINGS: &[(Key, BreakPointKind)] = &[
                (Key::Num1, BreakPointKind::_Ditch),
                (Key::D, BreakPointKind::_Ditch),
                (Key::Num2, BreakPointKind::_Unspecified),
                (Key::P, BreakPointKind::_Unspecified),
                (Key::Num3, BreakPointKind::_Berm),
                (Key::B, BreakPointKind::_Berm),
                (Key::Num4, BreakPointKind::_Limit),
                (Key::L, BreakPointKind::_Limit),
                (Key::Num7, BreakPointKind::_Step),
                (Key::S, BreakPointKind::_Step),
                (Key::Num9, BreakPointKind::_DoesNotDaylight),
                (Key::N, BreakPointKind::_DoesNotDaylight),
            ];
            ui.input(|i| {
                for (key, variant) in MAPPINGS {
                    if i.key_pressed(*key) {
                        *self.0 = *variant;
                        break;
                    }
                }
            });
        }
    }
    impl<'a> Widget for BreakPointKindSelector<'a> {
        fn ui(mut self, ui: &mut Ui) -> Response {
            use eframe::egui::ComboBox;
            let resp = ComboBox::from_id_salt("unique hash seed")
                .selected_text(self.0.to_string())
                .show_ui(ui, |ui| {
                    for variant in BreakPointKind::iter() {
                        ui.selectable_value(
                            self.0,
                            variant,
                            variant.to_string(),
                        );
                    }
                })
                .response;
            if resp.has_focus() {
                self.check_hotkeys(ui);
            }
            resp
        }
    }
}
