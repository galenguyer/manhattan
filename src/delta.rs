use crate::drink;
use std::fmt;

#[derive(Debug)]
pub struct Change {
    pub change_type: ChangeType,
    pub previous_machine: drink::Machine,
    pub previous_slot: drink::Slot,
    pub current_machine: drink::Machine,
    pub current_slot: drink::Slot,
}

impl std::fmt::Display for Change {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.change_type {
            ChangeType::SlotNowFull => {
                write!(
                    fmt,
                    "{}: Slot {} no longer empty, now contains {}",
                    self.current_machine.display_name,
                    self.current_slot.number,
                    self.current_slot.item.name
                )
            }
            ChangeType::SlotNowEmpty => {
                write!(
                    fmt,
                    "{}: Slot {} is empty (contained {})",
                    self.current_machine.display_name,
                    self.current_slot.number,
                    self.current_slot.item.name
                )
            }
            ChangeType::ItemNameChanged => {
                write!(
                    fmt,
                    "{}: Slot {} changed from {} to {}",
                    self.current_machine.display_name,
                    self.current_slot.number,
                    self.previous_slot.item.name,
                    self.current_slot.item.name
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum ChangeType {
    SlotNowEmpty,
    SlotNowFull,
    ItemNameChanged,
}
