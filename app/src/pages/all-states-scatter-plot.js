import React from 'react';
import { graphql } from 'gatsby';
import { AllStatesScatterPlotChart } from '../components/AllStatesScatterPlotChart';
import { Container } from '@material-ui/core';

const AllStatesScatterPlotPage = ({ data }) => {
  return (
    <main>
      <Container maxWidth="sm">
        <h1>State Equality Index 2020 - All States</h1>
        <AllStatesScatterPlotChart
          states={data.sei.states.edges}
          issues={data.sei.issues.edges} />
      </Container>
    </main>
  )
};

export default AllStatesScatterPlotPage;

export const query = graphql`
  query AllStatesScatterPlotPageQuery {
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