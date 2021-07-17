import React from 'react';
import { graphql } from 'gatsby';
import { ParallelCoordinatesPlotChart } from '../components/charts/ParallelCoordinatesPlotChart'

const ParallelPage = (props) => {
  const issues = props.data.sei.issues.edges.map((s) => s.node);
  return (
    <ParallelCoordinatesPlotChart issues={issues} />
  );
};

export default ParallelPage;

export const query = graphql`
query ParallelQuery {
  sei {
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
