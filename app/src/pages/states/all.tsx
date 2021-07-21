import React, { ReactElement } from "react"
import { PageProps, graphql } from "gatsby"
import { Container } from "@material-ui/core"
import { AllStatesRadialChart } from "../../features/states/all/AllStatesRadialChart"
import { Data } from "../../definitions/types"

const AllStatesPage = ({ data }: PageProps<Data>): ReactElement => {
  return (
    <main>
      <Container maxWidth="md">
        <h1>State policies on LGBTQ+ issues</h1>
        <AllStatesRadialChart {...data.sei} />
      </Container>
    </main>
  )
}

export default AllStatesPage

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
`
