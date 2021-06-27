use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use slab::Slab;

pub use model::QueryRoot;
pub mod model;

pub type HrcLgbtq2020Schema = Schema<model::QueryRoot, EmptyMutation, EmptySubscription>;

pub struct State {
    id: &'static str,
    name: &'static str,
    region: &'static str,
}

pub struct HrcLgbtq2020 {
    states: Slab<State>,
    state_data: HashMap<&'static str, usize>,
    issue_data: HashMap<&'static str, usize>,
}

impl HrcLgbtq2020 {
    pub fn new() -> Self {
        let mut states = Slab::new();
        let alabama = states.insert(State {
            id: "AL",
            name: "Alabama",
            region: "South",
        });

        let mut state_data = HashMap::new();

        state_data.insert("AL", alabama);

        Self {
            states,
            state_data,
            issue_data: HashMap::new(),
        }
    }

    pub fn state(&self, id: &str) -> Option<usize> {
        self.state_data.get(id).cloned()
    }
}
