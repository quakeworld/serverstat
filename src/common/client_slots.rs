#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents used / total / free slots on a server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientSlots {
    total: u32,
    used: u32,
    free: u32,
}

#[allow(dead_code)]
impl ClientSlots {
    pub fn new(used: u32, total: u32) -> Self {
        let free = total.saturating_sub(used);
        ClientSlots { total, used, free }
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn used(&self) -> u32 {
        self.used
    }

    pub fn free(&self) -> u32 {
        self.free
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_client_slots() {
        let client_slots = ClientSlots::new(5, 10);
        assert_eq!(client_slots.total(), 10);
        assert_eq!(client_slots.used(), 5);
        assert_eq!(client_slots.free(), 5);

        let client_slots = ClientSlots::new(0, 0);
        assert_eq!(client_slots.total(), 0);
        assert_eq!(client_slots.used(), 0);
        assert_eq!(client_slots.free(), 0);
    }
}
