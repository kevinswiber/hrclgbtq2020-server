import React from 'react';
import { graphql } from 'gatsby';
import { Container } from '@material-ui/core';

interface HomePageProps {
  data: {
    site: {
      siteMetadata: {
        title: string
      }
    }
  }
}

const HomePage = ({ data }: HomePageProps): JSX.Element => {
  return (
    <main>
      <Container maxWidth="md">
        <h1>{data.site.siteMetadata.title}</h1>
        <div className="divide-y divide-gray-100">
          <ul>
            <li>
              <a href="/all-states">All States</a>
            </li>
            <li>
              <a href="/all-states-scatter-plot">All States Scatterplot</a>
            </li>
            <li>
              <a href="/issues-bar-chart">Issues Bar Chart</a>
            </li>
            <li>
              <a href="/issues-by-state">Issues by State</a>
            </li>
          </ul>
        </div>
        </Container>
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