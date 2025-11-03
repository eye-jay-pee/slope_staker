use super::{BreakPoint, BreakPointKind};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeStake {
    pub pts: Rc<RefCell<Vec<BreakPoint>>>,
}
impl Serialize for SlopeStake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.pts.borrow().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SlopeStake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<BreakPoint>::deserialize(deserializer)?;
        Ok(SlopeStake {
            pts: Rc::new(RefCell::new(vec)),
        })
    }
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
    pub fn intercepts(a: &BreakPoint, b: &BreakPoint) -> bool {
        let run = b.offset - a.offset;
        let rise = b.elev - a.elev;
        a.slope * run == rise
    }
}

impl Default for SlopeStake {
    fn default() -> Self {
        let new_one = Self {
            pts: Rc::new(RefCell::new(Vec::new())),
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
        for pt in self.pts.borrow().iter() {
            writeln!(f, "\t{}", pt)?;
        }
        Ok(())
    }
}
