pub mod model;

use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use slab::Slab;

pub use model::{Issue, IssueKind, QueryRoot, Score, ScoreKind};

pub type HrcLgbtq2020Schema = Schema<model::QueryRoot, EmptyMutation, EmptySubscription>;

pub struct State {
    id: &'static str,
    name: &'static str,
    region: &'static str,
    district: &'static str,
    score: Score,
    issues: Vec<Issue>,
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
            district: "East South Central",
            score: Score {
                description: "High Priority to Achieve Basic Equality".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![Issue {
                name: "Transgender Healthcare".to_string(),
                kind: IssueKind::TransgenderHealthcare,
                description: "None".to_string(),
                value: 0,
            }],
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
