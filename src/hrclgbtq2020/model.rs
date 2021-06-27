use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Enum, Interface, Object, Result};

use super::HrcLgbtq2020;

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
    //region, district, score, anti_lgbtq_bills, issues
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

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn state(&self, ctx: &Context<'_>, id: String) -> Option<State> {
        ctx.data_unchecked::<HrcLgbtq2020>().state(&id).map(State)
    }
}
