use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum BreakPointKind {
    #[default]
    Centerline,
    Ditch,
    Unspecified,
    Berm,
    Limit,
    Step,
    DoesNotDaylight,
    Shoulder,
}
impl std::fmt::Display for BreakPointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BreakPointKind::*;
        write!(
            f,
            "{}",
            match self {
                Unspecified => "breakpoint",
                Ditch => "ditch",
                Berm => "berm",
                DoesNotDaylight => "does not daylight",
                Limit => "limit",
                Shoulder => "shoulder",
                Centerline => "crown",
                Step => "step",
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
                (Key::Num1, BreakPointKind::Ditch),
                (Key::D, BreakPointKind::Ditch),
                (Key::Num2, BreakPointKind::Unspecified),
                (Key::P, BreakPointKind::Unspecified),
                (Key::Num3, BreakPointKind::Berm),
                (Key::B, BreakPointKind::Berm),
                (Key::Num4, BreakPointKind::Limit),
                (Key::L, BreakPointKind::Limit),
                (Key::Num7, BreakPointKind::Step),
                (Key::S, BreakPointKind::Step),
                (Key::Num9, BreakPointKind::DoesNotDaylight),
                (Key::N, BreakPointKind::DoesNotDaylight),
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
            let resp = ComboBox::from_id_salt(("bp_kind", self.0 as *const _))
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
