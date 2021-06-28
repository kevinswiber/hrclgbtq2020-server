use std::{fs, io};

const DATA_FILE_PREFIX: &'static str = "use std::collections::HashMap;

use slab::Slab;

use crate::schema::{Issue, IssueKind, Score, ScoreKind, State};

pub struct Data;

impl Data {
    pub fn seed() -> (Slab<State>, HashMap<&'static str, usize>) {
        let mut states = Slab::new();
        let mut state_data = HashMap::new();
";
const DATA_FILE_SUFFIX: &'static str = "        (states, state_data)
    }
}";

pub struct Generator;
impl Generator {
    pub fn generate() -> io::Result<()> {
        let data = "        let alabama = states.insert(State {
            id: \"AL\",
            name: \"Alabama\",
            region: \"South\",
            district: \"East South Central\",
            score: Score {
                description: \"High Priority to Achieve Basic Equality\".to_string(),
                kind: ScoreKind::HighPriority,
            },
            issues: vec![Issue {
                name: \"Transgender Healthcare\".to_string(),
                kind: IssueKind::TransgenderHealthcare,
                description: \"None\".to_string(),
                value: 0,
            }],
        });
        state_data.insert(\"AL\", alabama);";

        let contents = format!("{}{}{}", DATA_FILE_PREFIX, data, DATA_FILE_SUFFIX);

        fs::write("./src/generated.rs", contents)?;

        println!("Complete");

        Ok(())
    }
}
