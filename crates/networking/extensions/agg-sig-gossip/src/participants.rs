use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Set of participants using PeerId
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantSet {
    /// Underlying storage as a HashSet
    participants: HashSet<PeerId>,
    /// Maximum capacity in terms of participant count
    max_participants: u16,
}

impl ParticipantSet {
    /// Create a new participant set with the given maximum capacity
    #[must_use]
    pub fn new(max_participants: u16) -> Self {
        Self {
            participants: HashSet::with_capacity(max_participants as usize),
            max_participants,
        }
    }

    /// Add a participant to the set
    pub fn add(&mut self, id: PeerId) -> bool {
        if self.participants.len() >= self.max_participants as usize {
            return false;
        }

        self.participants.insert(id)
    }

    /// Remove a participant from the set
    pub fn remove(&mut self, id: &PeerId) -> bool {
        self.participants.remove(id)
    }

    /// Check if a participant is in the set
    #[must_use]
    pub fn contains(&self, id: &PeerId) -> bool {
        self.participants.contains(id)
    }

    /// Get the number of participants in the set
    #[must_use]
    pub fn len(&self) -> usize {
        self.participants.len()
    }

    /// Check if the set is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.participants.is_empty()
    }

    /// Convert to a `HashSet`
    #[must_use]
    pub fn to_hashset(&self) -> HashSet<PeerId> {
        self.participants.clone()
    }

    /// Create from a `HashSet`
    #[must_use]
    pub fn from_hashset(set: &HashSet<PeerId>, max_participants: u16) -> Self {
        if set.len() > max_participants as usize {
            let mut limited_set = HashSet::with_capacity(max_participants as usize);
            for (i, peer_id) in set.iter().enumerate() {
                if i >= max_participants as usize {
                    break;
                }
                limited_set.insert(*peer_id);
            }

            Self {
                participants: limited_set,
                max_participants,
            }
        } else {
            Self {
                participants: set.clone(),
                max_participants,
            }
        }
    }

    /// Union with another set
    pub fn union(&mut self, other: &Self) {
        for peer_id in &other.participants {
            if self.participants.len() >= self.max_participants as usize {
                break;
            }
            self.participants.insert(*peer_id);
        }
    }

    /// Intersection with another set
    pub fn intersection(&mut self, other: &Self) {
        self.participants = self
            .participants
            .intersection(&other.participants)
            .cloned()
            .collect();
    }

    /// Difference from another set
    pub fn difference(&mut self, other: &Self) {
        self.participants = self
            .participants
            .difference(&other.participants)
            .cloned()
            .collect();
    }

    /// Iterate over participants in the set
    pub fn iter(&self) -> impl Iterator<Item = PeerId> + '_ {
        self.participants.iter().cloned()
    }
}
