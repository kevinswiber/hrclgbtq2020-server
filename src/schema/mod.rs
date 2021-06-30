pub mod model;

use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use slab::Slab;

use super::generated::Data;
pub use model::{Issue, IssueKind, QueryRoot, Score, ScoreKind};

pub type HrcLgbtq2020Schema = Schema<model::QueryRoot, EmptyMutation, EmptySubscription>;

pub struct State {
    pub id: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    pub district: &'static str,
    pub score: Score,
    pub issues: Vec<Issue>,
}

pub struct HrcLgbtq2020 {
    states: Slab<State>,
    state_data: HashMap<&'static str, usize>,
    //issue_data: HashMap<&'static str, usize>,
}

impl HrcLgbtq2020 {
    pub fn new() -> Self {
        let (states, state_data) = Data::seed();

        Self {
            states,
            state_data,
            //issue_data: HashMap::new(),
        }
    }

    pub fn state(&self, id: &str) -> Option<usize> {
        self.state_data.get(id).cloned()
    }
}
