pub mod elev;
pub mod kind;
pub mod offset;
pub mod slope;

use elev::Elevation;
use offset::Offset;
use slope::Slope;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BreakPoint {
    kind: kind::BreakPointKind,
    _cut_or_fill_from_slope_stake: Option<f32>,

    idx: usize,

    slope: Slope,
    elev: Elevation,
    offset: Offset,
}
impl BreakPoint {
    fn get_prev<'a>(&self, all: &'a [BreakPoint]) -> Option<&'a BreakPoint> {
        all.get(self.idx - 1)
    }
    fn get_next<'a>(&self, all: &'a [BreakPoint]) -> Option<&'a BreakPoint> {
        all.get(self.idx + 1)
    }
    fn try_infer_slope_val<'a>(
        &self,
        all: Option<&'a [BreakPoint]>,
    ) -> Option<f64> {
        let next = self.get_next(all?)?;

        let next_os = next.get_offset(all).read()?;
        let curr_os = self.get_offset(all).read()?;
        let os_diff = next_os - curr_os;

        let next_elev = next.get_elev(all).read()?;
        let curr_elev = self.get_elev(all).read()?;
        let elev_diff = next_elev - curr_elev;

        Some(os_diff / elev_diff)
    }
    pub fn get_slope<'a>(&self, all: Option<&'a [BreakPoint]>) -> Slope {
        if self.slope.read() == None {
            if let Some(v) = self.try_infer_slope_val(all) {
                return Slope::from(v);
            }
        }
        self.slope
    }
    fn try_infer_elev_val<'a>(
        &self,
        all: Option<&'a [BreakPoint]>,
    ) -> Option<f64> {
        let prev = self.get_prev(all?)?;
        let prev_os = prev.get_offset(all).read()?;
        let prev_slope = prev.get_slope(all).read()?;
        let curr_os = self.get_offset(all).read()?;
        let os_diff = curr_os - prev_os;
        let prev_elev = prev.get_elev(all).read()?;
        let elev_diff = os_diff / prev_slope;
        Some(prev_elev + elev_diff)
    }
    pub fn get_elev<'a>(&self, all: Option<&'a [BreakPoint]>) -> Elevation {
        if self.elev.read() == None {
            if let Some(v) = self.try_infer_elev_val(all) {
                return Elevation::from(v);
            }
        }
        self.elev
    }
    fn try_infer_offset_val<'a>(
        &self,
        all: Option<&'a [BreakPoint]>,
    ) -> Option<f64> {
        let prev = self.get_prev(all?)?;
        let prev_ht = prev.get_elev(all).read()?;
        let prev_slp = prev.get_slope(all).read()?;
        let curr_ht = self.get_elev(all).read()?;
        let height_diff = curr_ht - prev_ht;
        let offset_diff = height_diff * prev_slp;

        Some(prev_ht + offset_diff)
    }
    pub fn get_offset<'a>(&self, all: Option<&'a [BreakPoint]>) -> Offset {
        if self.offset.read() == None {
            if let Some(v) = self.try_infer_offset_val(all) {
                return Offset::from(v);
            }
        }
        self.offset
    }
}

impl std::fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blank = "____";
        let offset_txt = match self.get_offset(None).read() {
            Some(os) => os.to_string(),
            None => String::from(blank),
        };
        let elev_txt = match self.get_elev(None).read() {
            Some(e) => e.to_string(),
            None => String::from(blank),
        };
        let slope_txt = match self.get_slope(None).read() {
            Some(s) => s.to_string(),
            None => String::from(blank),
        };

        write!(
            f,
            "{} @ {} L of CL @ {}. Then {} slope elev...",
            self.kind, offset_txt, elev_txt, slope_txt,
        )
    }
}

pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct BreakPointEditor<'a>(&'a mut BreakPoint);
    impl<'a> BreakPointEditor<'a> {
        pub fn new(bp: &'a mut BreakPoint) -> Self {
            Self(bp)
        }
    }
    impl<'a> Widget for BreakPointEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            use super::elev::ui::ElevationEditor;
            use super::kind::ui::BreakPointKindSelector;
            use super::offset::ui::OffsetEditor;
            use super::slope::ui::SlopeEditor;

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(BreakPointKindSelector::new(&mut self.0.kind));
                    ui.add(SlopeEditor::new(&mut self.0.slope));
                });
                ui.horizontal(|ui| {
                    ui.add(OffsetEditor::new(&mut self.0.offset));
                    ui.add(ElevationEditor::new(&mut self.0.elev));
                });
            })
            .response
        }
    }
}
