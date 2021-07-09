import React from 'react';
import { graphql } from 'gatsby';
import { IssuesByStateBarChart } from '../components/IssuesByStateBarChart';

const IssuesByStatePage = (props) => {
  const states = props.data.sei.states.edges.map((s) => s.node);
  const current = props.location.hash.length ?
    props.location.hash.slice(1).toUpperCase() :
    null;
  console.log(current);

  return (
    <main>
      <div className="md:container md:mx-auto">
        <h1>State Equality Index 2020 - All States</h1>
        <IssuesByStateBarChart
          states={states}
          current={current} />
      </div>
    </main>
  )
};

export default IssuesByStatePage;

export const query = graphql`
query IssuesByStateQuery {
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
  }
}
`;