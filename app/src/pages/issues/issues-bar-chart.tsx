import React from "react";
import { PageProps, graphql } from "gatsby";
//import { IssuesBarChart } from '../components/charts/IssuesBarChart';
import { Container } from "@material-ui/core";
import { StateEqualityIndex } from "../../definitions/types";

interface Data {
  sei: StateEqualityIndex;
}

const IssuesBarChartPage = ({ data }: PageProps<Data>): React.ReactElement => {
  const states = data.sei.states.edges.map(s => s.node);
  //const issues = data.sei.issues.edges.map((i) => i.node);
  console.log(states);

  return (
    <main>
      <Container maxWidth="lg">
        <h1>State Equality Index 2020 - All States</h1>
        {/*<IssuesBarChart
          states={states}
        issues={issues} />*/}
      </Container>
    </main>
  );
};

export default IssuesBarChartPage;

export const query = graphql`
  query IssuesBarChartPageQuery {
    sei {
      states {
        edges {
          node {
            id
            name
            region
            district
            score {
              kind
              description
            }
            issues {
              kind
              name
              policy
              value
            }
          }
        }
      }
      issues {
        edges {
          node {
            id
            name
            states {
              id
              name
              policy
              value
            }
          }
        }
      }
    }
  }
`;
