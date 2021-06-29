use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Enum, Object, Result, SimpleObject};

use super::HrcLgbtq2020;

#[derive(SimpleObject, Clone)]
pub struct Score {
    pub kind: ScoreKind,
    pub description: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ScoreKind {
    Innovative,
    Solidifying,
    Building,
    HighPriority,
}

#[derive(SimpleObject, Clone)]
pub struct Issue {
    pub name: String,
    pub kind: IssueKind,
    pub description: String,
    pub value: i8,
}

/// A category of reported issue.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum IssueKind {
    All,
    TransgenderHealthcare,
    SchoolAntiBullying,
    PublicAccommodations,
    MarriageEquality,
    Housing,
    HateCrimes,
    GenderMarkerUpdatesOnIdentification,
    Employment,
    Education,
    DiscriminationInChildWelfare,
    AntiConversionTherapy,
}

pub struct State(usize);

#[Object]
impl State {
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0].id
    }

    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0].name
    }
    async fn region(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0].region
    }
    async fn district(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0].district
    }
    async fn score(&self, ctx: &Context<'_>) -> Score {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0]
            .score
            .clone()
    }
    async fn issues(&self, ctx: &Context<'_>) -> Vec<Issue> {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0]
            .issues
            .clone()
    }

    //region, district, score, anti_lgbtq_bills, issues
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn state(&self, ctx: &Context<'_>, id: String) -> Option<State> {
        ctx.data_unchecked::<HrcLgbtq2020>().state(&id).map(State)
    }

    async fn all_states(&self, ctx: &Context<'_>) -> Vec<State> {
        ctx.data_unchecked::<HrcLgbtq2020>()
            .state_data
            .values()
            .copied()
            .map(|id| State(id))
            .collect::<Vec<State>>()
    }
}
