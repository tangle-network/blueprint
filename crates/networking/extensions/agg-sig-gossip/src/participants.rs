use bitvec::prelude::*;
use gadget_networking::types::ParticipantId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Efficient representation of participants using bitvec
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantSet {
    /// Bit vector representation for quick membership checks
    bitvec: BitVec,
    /// Maximum participant ID
    max_id: u16,
}

impl ParticipantSet {
    /// Create a new participant set
    pub fn new(max_id: u16) -> Self {
        Self {
            bitvec: bitvec![0; max_id as usize + 1],
            max_id,
        }
    }

    /// Add a participant to the set
    pub fn add(&mut self, id: ParticipantId) -> bool {
        if id.0 > self.max_id {
            return false;
        }

        let was_present = self.bitvec[id.0 as usize];
        self.bitvec.set(id.0 as usize, true);
        !was_present
    }

    /// Remove a participant from the set
    pub fn remove(&mut self, id: ParticipantId) -> bool {
        if id.0 > self.max_id {
            return false;
        }

        let was_present = self.bitvec[id.0 as usize];
        self.bitvec.set(id.0 as usize, false);
        was_present
    }

    /// Check if a participant is in the set
    pub fn contains(&self, id: &ParticipantId) -> bool {
        if id.0 > self.max_id {
            return false;
        }

        self.bitvec[id.0 as usize]
    }

    /// Get the number of participants in the set
    pub fn len(&self) -> usize {
        self.bitvec.count_ones()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to a HashSet
    pub fn to_hashset(&self) -> HashSet<ParticipantId> {
        let mut result = HashSet::with_capacity(self.len());
        for i in 0..=self.max_id as usize {
            if self.bitvec[i] {
                result.insert(ParticipantId(i as u16));
            }
        }
        result
    }

    /// Create from a HashSet
    pub fn from_hashset(set: &HashSet<ParticipantId>, max_id: u16) -> Self {
        let mut result = Self::new(max_id);
        for &id in set {
            result.add(id);
        }
        result
    }

    /// Union with another set
    pub fn union(&mut self, other: &Self) {
        assert_eq!(self.max_id, other.max_id, "Sets must have the same max_id");
        self.bitvec |= &other.bitvec;
    }

    /// Intersection with another set
    pub fn intersection(&mut self, other: &Self) {
        assert_eq!(self.max_id, other.max_id, "Sets must have the same max_id");
        self.bitvec &= &other.bitvec;
    }

    /// Difference from another set
    pub fn difference(&mut self, other: &Self) {
        assert_eq!(self.max_id, other.max_id, "Sets must have the same max_id");
        self.bitvec &= !other.bitvec.clone();
    }

    /// Iterate over participants in the set
    pub fn iter(&self) -> impl Iterator<Item = ParticipantId> + '_ {
        self.bitvec.iter_ones().map(|idx| ParticipantId(idx as u16))
    }
}
