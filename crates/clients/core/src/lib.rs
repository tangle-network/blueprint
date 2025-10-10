pub mod error;
use error::Error;

use blueprint_std::hash::Hash;
use auto_impl::auto_impl;

pub type OperatorSet<K, V> = std::collections::BTreeMap<K, V>;

#[auto_impl::auto_impl(&, Arc)]
pub trait BlueprintServicesClient: Send + Sync + 'static {
    /// The ID of for operators at the blueprint/application layer. Typically a cryptograpgic key in the form of a point on
    /// some elliptic curve, e.g., an ECDSA public key (point). However, this is not required.
    type PublicApplicationIdentity: Eq + PartialEq + Hash + Ord + PartialOrd + Send + Sync + 'static;
    /// The ID of the operator's account, not necessarily associated with the `PublicApplicationIdentity`,
    /// but may be cryptographically related thereto. E.g., AccountId32
    type PublicAccountIdentity: Send + Sync + 'static;
    /// A generalized ID that distinguishes the current blueprint from others
    type Id: Send + Sync + 'static;
    type Error: core::error::Error + From<Error> + Send + Sync + 'static;

    /// Returns the set of operators for the current job
    async fn get_operators(
        &self,
    ) -> Result<
        OperatorSet<Self::PublicAccountIdentity, Self::PublicApplicationIdentity>,
        Self::Error,
    >;
    /// Returns the ID of the operator
    async fn operator_id(&self) -> Result<Self::PublicApplicationIdentity, Self::Error>;
    /// Returns the unique ID for this blueprint
    async fn blueprint_id(&self) -> Result<Self::Id, Self::Error>;

    /// Returns an operator set with the index of the current operator within that set
    async fn get_operators_and_operator_id(
        &self,
    ) -> Result<(OperatorSet<usize, Self::PublicApplicationIdentity>, usize), Self::Error> {
        let operators = self
            .get_operators()
            .await
            .map_err(|e| Error::GetOperatorsAndOperatorId(e.to_string()))?;
        let my_id = self
            .operator_id()
            .await
            .map_err(|e| Error::GetOperatorsAndOperatorId(e.to_string()))?;
        let mut ret = OperatorSet::new();
        let mut ret_id = None;
        for (id, op) in operators.into_values().enumerate() {
            if my_id == op {
                ret_id = Some(id);
            }

            ret.insert(id, op);
        }

        let ret_id = ret_id.ok_or_else(|| {
            Error::GetOperatorsAndOperatorId("Operator index not found".to_string())
        })?;
        Ok((ret, ret_id))
    }

    /// Returns the index of the current operator in the operator set
    async fn get_operator_index(&self) -> Result<usize, Self::Error> {
        let (_, index) = self
            .get_operators_and_operator_id()
            .await
            .map_err(|err| Error::GetOperatorIndex(err.to_string()))?;
        Ok(index)
    }
}

#[auto_impl(Arc)]
pub trait EventsClient<Event>: Clone + Send + Sync {
    /// Fetch the next event from the client.
    async fn next_event(&self) -> Option<Event>;
    /// Fetch the latest event from the client.
    ///
    /// If no event has yet been fetched, the client will call [`next_event`](Self::next_event).
    async fn latest_event(&self) -> Option<Event>;
}
