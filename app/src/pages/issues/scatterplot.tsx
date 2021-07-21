import React, { ReactElement } from "react"
import { PageProps, graphql } from "gatsby"
import { AllStatesScatterPlotChart } from "../../features/issues/scatterplot/AllStatesScatterPlotChart"
import { Container } from "@material-ui/core"
import { Data } from "../../definitions/types"

const AllStatesScatterPlotPage = ({ data }: PageProps<Data>): ReactElement => {
  const states = data.sei.states.edges.map(s => s.node)
  const issues = data.sei.issues.edges.map(i => i.node)

  return (
    <main>
      <Container maxWidth="lg">
        <h1>State Equality Index 2020 - All States</h1>
        <AllStatesScatterPlotChart states={states} issues={issues} />
      </Container>
    </main>
  )
}

export default AllStatesScatterPlotPage

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
`
