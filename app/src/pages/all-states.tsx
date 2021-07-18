import React from 'react';
import { graphql } from 'gatsby';
import { Container } from '@material-ui/core'
import { AllStatesRadialChart } from '../components/charts/AllStatesRadialChart';
import { StateEqualityIndex } from '../types';

interface IssuesByStatePageProps {
  data: { sei: StateEqualityIndex }
}

const AllStatesPage = ({ data }: IssuesByStatePageProps): JSX.Element => {
  return (
    <main>
      <Container maxWidth="md">
        <h1>State policies on LGBTQ+ issues</h1>
        <AllStatesRadialChart
          states={data.sei.states.edges}
          issues={data.sei.issues.edges} />
      </Container>
    </main>
  )
};

export default AllStatesPage;

export const query = graphql`
  query AllStatesPageQuery {
    sei {
      states {
        edges {
          node {
            id
            name
            region
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
              value
            }
          }
        }
      }
    }
  }
`;