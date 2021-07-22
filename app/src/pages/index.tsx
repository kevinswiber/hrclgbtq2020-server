import React, { ReactElement } from "react";
import { graphql, Link } from "gatsby";
import { Container } from "@material-ui/core";

interface HomePageProps {
  data: {
    site: {
      siteMetadata: {
        title: string;
      };
    };
  };
}

const HomePage = ({ data }: HomePageProps): ReactElement => {
  return (
    <main>
      <Container maxWidth="md">
        <h1>{data.site.siteMetadata.title}</h1>
        <div className="divide-y divide-gray-100">
          <ul>
            <li>
              <Link to="/states/all">States</Link>
            </li>
            <li>
              <Link to="/issues/issues-by-state">Issues by State</Link>
            </li>
            <li>
              <Link to="/issues/scatterplot">Issues Scatterplot</Link>
            </li>
            <li>
              <Link to="/issues/issues-bar-chart">Issues Bar Chart</Link>
            </li>
          </ul>
        </div>
      </Container>
    </main>
  );
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
