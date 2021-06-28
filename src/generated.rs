use std::collections::HashMap;

use slab::Slab;

use crate::schema::{Issue, IssueKind, Score, ScoreKind, State};

pub struct Data;

impl Data {
    pub fn seed() -> (Slab<State>, HashMap<&'static str, usize>) {
        let mut states = Slab::new();
        let mut state_data = HashMap::new();
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
        state_data.insert("AL", alabama);        (states, state_data)
    }
}