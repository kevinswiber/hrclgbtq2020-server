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

    async fn states(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, State, EmptyFields, EmptyFields>> {
        let states = ctx
            .data_unchecked::<HrcLgbtq2020>()
            .state_data
            .values()
            .copied()
            .collect::<Vec<_>>();
        query_states(after, before, first, last, &states)
            .await
            .map(|conn| conn.map_node(State))
    }
}

async fn query_states(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    states: &[usize],
) -> Result<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = states.len();

            if let Some(after) = after {
                if after >= states.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &states[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < states.len());
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );
            Ok(connection)
        },
    )
    .await
}
