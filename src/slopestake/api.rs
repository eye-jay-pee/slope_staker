use super::{BreakPoint, BreakPointKind, Station};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeStake {
    sta: Station,
    pub pts: Rc<RefCell<Vec<BreakPoint>>>,
}

impl SlopeStake {
    pub fn remove_point_at(&mut self, index: usize) {
        self.pts.borrow_mut().remove(index);
    }
    pub fn add_point_at(&mut self, point: BreakPoint, index: usize) {
        if (1..self.pts.borrow().len()).contains(&index) {
            self.pts.borrow_mut().insert(index, point);
        } else {
            println!("invalid insertion index: {index}");
        }
    }
    pub fn is_valid(&self) -> bool {
        for window in self.pts.borrow().windows(2) {
            let prev = &window[0];
            let next = &window[1];
            if prev.elev + prev.slope * (next.offset - prev.offset) != next.elev
            {
                return false;
            }
        }
        true
    }
}

impl Default for SlopeStake {
    fn default() -> Self {
        let new_one = Self {
            pts: Rc::new(RefCell::new(Vec::new())),
            sta: Station::default(),
        };

        new_one
            .pts
            .borrow_mut()
            .push(BreakPoint::new(BreakPointKind::Crown));
        new_one
            .pts
            .borrow_mut()
            .push(BreakPoint::new(BreakPointKind::Limit));

        new_one
    }
}

impl std::fmt::Display for SlopeStake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.sta)?;
        for pt in self.pts.borrow().iter() {
            writeln!(f, "\t{}", pt)?;
        }
        Ok(())
    }
}
