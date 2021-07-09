import React from 'react';
import { graphql } from 'gatsby';
import { RadialChartDeclarative } from '../components/RadialChartDeclarative';

const AllStatesDeclarativePage = ({ data }) => {
  return (
    <main>
      <div className="md:container md:mx-auto">
        <h1>State Equality Index 2020 - All States</h1>
        <RadialChartDeclarative
          states={data.sei.states.edges}
          issues={data.sei.issues.edges} />
      </div>
    </main>
  )
};

export default AllStatesDeclarativePage;

export const query = graphql`
  query AllStatesDeclarativePageQuery {
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