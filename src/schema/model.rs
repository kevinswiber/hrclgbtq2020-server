use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Enum, Object, Result, SimpleObject};

use super::HrcLgbtq2020;

#[derive(SimpleObject, Clone)]
pub(crate) struct Score {
    pub(crate) kind: ScoreKind,
    pub(crate) description: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ScoreKind {
    Innovative,
    Solidifying,
    Building,
    HighPriority,
}

#[derive(SimpleObject, Clone)]
pub(crate) struct StateIssue {
    pub(crate) name: String,
    pub(crate) kind: IssueKind,
    pub(crate) policy: String,
    pub(crate) value: i8,
}

#[derive(SimpleObject, Clone)]
pub(crate) struct IssueState {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) policy: String,
    pub(crate) value: i8,
}

/// A category of reported issue.
#[derive(Enum, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum IssueKind {
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

pub(crate) struct State(usize);

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

    async fn issues(&self, ctx: &Context<'_>) -> Vec<StateIssue> {
        ctx.data_unchecked::<HrcLgbtq2020>().states[self.0]
            .issues
            .clone()
    }
}

pub(crate) struct Issue(usize);

#[Object]
impl Issue {
    async fn id(&self, ctx: &Context<'_>) -> IssueKind {
        ctx.data_unchecked::<HrcLgbtq2020>().issues[self.0].id
    }

    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<HrcLgbtq2020>().issues[self.0].name
    }

    async fn states(&self, ctx: &Context<'_>) -> Vec<IssueState> {
        ctx.data_unchecked::<HrcLgbtq2020>().issues[self.0]
            .states
            .clone()
    }
}

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn state(&self, ctx: &Context<'_>, id: String) -> Option<State> {
        ctx.data_unchecked::<HrcLgbtq2020>().state(&id).map(State)
    }

    async fn issue(&self, ctx: &Context<'_>, id: IssueKind) -> Option<Issue> {
        ctx.data_unchecked::<HrcLgbtq2020>().issue(&id).map(Issue)
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
        query_connections(after, before, first, last, &states)
            .await
            .map(|conn| conn.map_node(State))
    }

    async fn issues(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Issue, EmptyFields, EmptyFields>> {
        let issues = ctx
            .data_unchecked::<HrcLgbtq2020>()
            .issue_data
            .values()
            .copied()
            .collect::<Vec<_>>();
        query_connections(after, before, first, last, &issues)
            .await
            .map(|conn| conn.map_node(Issue))
    }
}

async fn query_connections(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    connections: &[usize],
) -> Result<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = connections.len();

            if let Some(after) = after {
                if after >= connections.len() {
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

            let mut slice = &connections[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < connections.len());
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
