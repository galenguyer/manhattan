use crate::drink;
use std::fmt;

#[derive(Debug)]
pub struct Change {
    pub change_type: ChangeType,
    pub machine: drink::Machine,
    pub slot: drink::Slot,
    pub item: drink::Item,
}

impl std::fmt::Display for Change {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.change_type {
            ChangeType::SlotNowFull => {
                write!(fmt, "{}: Slot {} no longer empty, now contains {}", self.machine.display_name, self.slot.number, self.item.name)
            }
            ChangeType::SlotNowEmpty => {
                write!(fmt, "{}: Slot {} is empty (contained {})", self.machine.display_name, self.slot.number, self.item.name)
            }
            _ => {
                write!(fmt, "Unknown Change")
            }
        }
    }
}

#[derive(Debug)]
pub enum ChangeType {
    SlotNowEmpty,
    SlotNowFull,
}