pub(crate) mod model;

use std::collections::HashMap;

use slab::Slab;

use self::model::IssueState;

use super::generated::Data;
pub(crate) use model::{IssueKind, QueryRoot, Score, ScoreKind, StateIssue};

//pub(crate) type HrcLgbtq2020Schema = Schema<model::QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) struct State {
    pub(crate) id: &'static str,
    pub(crate) name: &'static str,
    pub(crate) region: &'static str,
    pub(crate) district: &'static str,
    pub(crate) score: Score,
    pub(crate) issues: Vec<StateIssue>,
}

pub(crate) struct Issue {
    pub(crate) id: IssueKind,
    pub(crate) name: &'static str,
    pub(crate) states: Vec<IssueState>,
}

pub(crate) struct HrcLgbtq2020 {
    states: Slab<State>,
    issues: Slab<Issue>,
    state_data: HashMap<&'static str, usize>,
    issue_data: HashMap<IssueKind, usize>,
}

impl HrcLgbtq2020 {
    pub(crate) fn new() -> Self {
        let (states, state_data) = Data::seed();

        let mut issues = Slab::new();
        let mut issue_data = HashMap::new();

        let transgender_healthcare = issues.insert(Issue {
            id: IssueKind::TransgenderHealthcare,
            name: "Transgender Healthcare",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::TransgenderHealthcare, transgender_healthcare);

        let school_anti_bullying = issues.insert(Issue {
            id: IssueKind::SchoolAntiBullying,
            name: "School Anti-Bullying",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::SchoolAntiBullying, school_anti_bullying);

        let public_accommodations = issues.insert(Issue {
            id: IssueKind::PublicAccommodations,
            name: "Public Accommodations",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::PublicAccommodations, public_accommodations);

        for (_, state) in &states {
            for issue in &state.issues {
                if let Some((_, iss)) = issues.iter_mut().find(|(_, iss)| iss.id == issue.kind) {
                    iss.states.push(IssueState {
                        id: state.id.to_string(),
                        name: state.name.to_string(),
                        policy: issue.policy.to_string(),
                        value: issue.value,
                    })
                }
            }
        }

        /*
                issue_data.insert(IssueKind::TransgenderHealthcare, HashMap::new());
                issue_data.insert(IssueKind::SchoolAntiBullying, HashMap::new());
                issue_data.insert(IssueKind::PublicAccommodations, HashMap::new());
                issue_data.insert(IssueKind::MarriageEquality, HashMap::new());
                issue_data.insert(IssueKind::Housing, HashMap::new());
                issue_data.insert(IssueKind::HateCrimes, HashMap::new());
                issue_data.insert(
                    IssueKind::GenderMarkerUpdatesOnIdentification,
                    HashMap::new(),
                );
                issue_data.insert(IssueKind::Employment, HashMap::new());
                issue_data.insert(IssueKind::Education, HashMap::new());
                issue_data.insert(IssueKind::DiscriminationInChildWelfare, HashMap::new());
                issue_data.insert(IssueKind::AntiConversionTherapy, HashMap::new());

                for (key, value) in state_data {
                    match states.get(value) {
                        None => continue,
                        Some(state) => {
                            for issue in state.issues {
                                if let Some(inner) = issue_data.get_mut(&issue.kind) {
                                    if let Some(mut v) = inner.get(&issue.policy) {
                                        v.push(issue);
                                    } else {
                                        inner.insert(issue.policy, vec![issue]);
                                    }
                                } else {
                                    let mut inner_hash = HashMap::new();
                                    inner_hash.insert(issue.policy, vec![issue]);
                                    issue_data.insert(issue.kind, inner_hash);
                                }
                            }
                        }
                    }
                }

                //issues_data["Transgender Healthcare"]["None"] = Vec<State>
        */
        Self {
            states,
            state_data,
            issues,
            issue_data,
        }
    }

    pub(crate) fn issue(&self, id: &IssueKind) -> Option<usize> {
        self.issue_data.get(id).cloned()
    }

    pub(crate) fn state(&self, id: &str) -> Option<usize> {
        self.state_data.get(id).cloned()
    }
}
