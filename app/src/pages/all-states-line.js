import React from 'react';
import { graphql } from 'gatsby';
import { LineChart } from '../components/LineChart';

const AllStatesLinePage = ({ data }) => {
  return (
    <main>
      <div className="md:container md:mx-auto">
        <h1>State Equality Index 2020 - All States</h1>
        <LineChart
          states={data.sei.states.edges} />
      </div>
    </main>
  )
};

export default AllStatesLinePage;

export const query = graphql`
  query AllStatesLinePageQuery {
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
    }
  }
`;