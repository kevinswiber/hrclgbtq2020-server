import React from 'react';
import { graphql } from 'gatsby';

const HomePage = ({ data }) => {
  return (
    <main>
      <div className="md:container md:mx-auto">
        <h1>{data.site.siteMetadata.title}</h1>
        <div className="divide-y divide-gray-100">
          <ul>
            <li>
              <a href="/all-states">All States</a>
            </li>
            <li>
              <a href="/issues-by-state">Issues by State</a>
            </li>
          </ul>
        </div>
      </div>
    </main>
  )
};

export default HomePage;

export const query = graphql`
  query HomePageQuery {
    site {
      siteMetadata {
        title
      }
    }
  }
`;