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

        let marriage_equality = issues.insert(Issue {
            id: IssueKind::MarriageEquality,
            name: "Marriage Equality & Relationship Recognition",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::MarriageEquality, marriage_equality);

        let housing = issues.insert(Issue {
            id: IssueKind::Housing,
            name: "Housing",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::Housing, housing);

        let hate_crimes = issues.insert(Issue {
            id: IssueKind::HateCrimes,
            name: "Hate Crimes",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::HateCrimes, hate_crimes);

        let gender_marker_updates = issues.insert(Issue {
            id: IssueKind::GenderMarkerUpdatesOnIdentification,
            name: "Gender Marker Updates on Identification Documents",
            states: Vec::new(),
        });
        issue_data.insert(
            IssueKind::GenderMarkerUpdatesOnIdentification,
            gender_marker_updates,
        );

        let employment = issues.insert(Issue {
            id: IssueKind::Employment,
            name: "Employment",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::Employment, employment);

        let education = issues.insert(Issue {
            id: IssueKind::Education,
            name: "Education",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::Education, education);

        let discrimination = issues.insert(Issue {
            id: IssueKind::DiscriminationInChildWelfare,
            name: "Discrimination in Child Welfare Services",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::DiscriminationInChildWelfare, discrimination);

        let anti_conversion = issues.insert(Issue {
            id: IssueKind::AntiConversionTherapy,
            name: "Anti-Conversion Therapy",
            states: Vec::new(),
        });
        issue_data.insert(IssueKind::AntiConversionTherapy, anti_conversion);

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
