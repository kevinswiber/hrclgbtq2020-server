import React from 'react';
import { graphql } from 'gatsby';
import { RadialChart } from '../components/RadialChart';

const AllStatesPage = ({ data }) => {
    return (
        <main>
            <div className="md:container md:mx-auto">
                <h1>State Equality Index 2020 - All States</h1>
                <RadialChart
                    states={data.sei.states.edges}
                    issues={data.sei.issues.edges} />
            </div>
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