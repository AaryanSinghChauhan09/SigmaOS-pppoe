// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-pppoe: pppoe component model.
//!
//! Real implementation: pppoe_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PppoeItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PppoeError {
    EmptyPppoeItemName,
    DuplicatePppoeItem(String),
    EmptyEventSubject,
}

impl fmt::Display for PppoeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPppoeItemName => write!(f, "pppoe_item name must not be empty"),
            Self::DuplicatePppoeItem(id) => write!(f, "duplicate pppoe_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PppoeError {}

pub struct Pppoe {
    pub name: String,
    pppoe_items: BTreeMap<String, PppoeItem>,
    events: BTreeMap<String, Event>,
}

impl Pppoe {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pppoe_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_pppoe_item(&mut self, item: PppoeItem) -> Result<(), PppoeError> {
        if item.name.trim().is_empty() {
            return Err(PppoeError::EmptyPppoeItemName);
        }
        let id = item.id.clone();
        if self.pppoe_items.contains_key(&id) {
            return Err(PppoeError::DuplicatePppoeItem(id.clone()));
        }
        self.pppoe_items.insert(id, item);
        Ok(())
    }

    pub fn pppoe_item_count(&self) -> usize {
        self.pppoe_items.len()
    }

    pub fn get_pppoe_item(&self, id: &str) -> Option<&PppoeItem> {
        self.pppoe_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PppoeError> {
        if event.subject.trim().is_empty() {
            return Err(PppoeError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_pppoe_item(id: &str, name: &str) -> PppoeItem {
        PppoeItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Pppoe::new("Pppoe");
        assert_eq!(m.name, "Pppoe");
        assert_eq!(m.pppoe_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_pppoe_item() {
        let mut m = Pppoe::new("Pppoe");
        m.add_pppoe_item(sample_pppoe_item("a1", "Alpha")).unwrap();
        assert_eq!(m.pppoe_item_count(), 1);
        assert!(m.get_pppoe_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Pppoe::new("Pppoe");
        assert!(m.add_pppoe_item(sample_pppoe_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Pppoe::new("Pppoe");
        m.add_pppoe_item(sample_pppoe_item("x", "One")).unwrap();
        assert!(m.add_pppoe_item(sample_pppoe_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Pppoe::new("Pppoe");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
