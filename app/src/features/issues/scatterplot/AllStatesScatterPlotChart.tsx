import React from "react"
import * as d3 from "d3"
import {
  Orientation,
  BandTick,
  TickLine,
  TickText,
  NumberTick,
} from "../../../common/d3/Axis"

export const AllStatesScatterPlotChart = (props): React.ReactElement => {
  const width = 620
  const height = 400
  const margin = { top: 30, right: 270, bottom: 20, left: 260 }
  //const margin = { top: 0, right: 0, bottom: 0, left: 0 };

  const issues: { [id: string]: string } = {}

  //const issues = prop.issues.map((issue) => {

  //})
  for (const issue of props.issues) {
    /*if (issues[issue.node.id]) {
      if issues[issue.node.id]
    }*/
    issues[issue.id] = issue.name
  }

  const states = props.states
    .map(s => s)
    .sort((a, b) => d3.ascending(a.id, b.id))

  const categories = [
    "TRANSGENDER_HEALTHCARE",
    "SCHOOL_ANTI_BULLYING",
    "PUBLIC_ACCOMMODATIONS",
    "MARRIAGE_EQUALITY",
    "HOUSING",
    "HATE_CRIMES",
    "GENDER_MARKER_UPDATES_ON_IDENTIFICATION",
    "EMPLOYMENT",
    "EDUCATION",
    "DISCRIMINATION_IN_CHILD_WELFARE",
    "ANTI_CONVERSION_THERAPY",
  ].map(n => issues[n])

  const y = d3
    .scalePoint()
    .domain(categories)
    .range([0, height - margin.bottom])
    .padding(1)

  const x = d3
    .scaleLinear()
    .domain([-6, 6])
    .range([0, width - margin.right])

  const flattened = states.flatMap(row => {
    return categories.map(category => {
      const issue = row.issues.find(iss => iss.name === category)
      return {
        state: row.name,
        abbreviation: row.id,
        region: row.region,
        category,
        status: issue.policy,
        value: issue.value,
      }
    })
  })
  // issue[value] = [{state, abbreviatoin, region, category, stat}]

  const xAxisOrientation = Orientation.BOTTOM
  const yAxisOrientation = Orientation.LEFT

  return (
    <div id="all-states-scatter-plot">
      <svg viewBox={`0, 0, ${width}, ${height}`}>
        <g stroke="currentColor" strokeOpacity="0.1">
          {x.ticks().map(d => {
            return (
              <line
                x1={x(d) + margin.left + 0.5}
                x2={x(d) + margin.left + 0.5}
                y1={margin.top}
                y2={height - margin.bottom}
              />
            )
          })}
          {categories.map(d => {
            return (
              <line
                y1={(y(d) || 0) + margin.top}
                y2={(y(d) || 0) + margin.top}
                x1={margin.left}
                x2={width}
              />
            )
          })}
        </g>
        <g
          transform={`translate(${margin.left}, ${margin.top})`}
          /*transform={`translate(0,${height - margin.top - margin.bottom})`}*/
          fill="none"
          fontSize="12"
          fontFamily="sans-serif"
          textAnchor="end"
        >
          {y.domain().map(issue => {
            return (
              <BandTick orient={yAxisOrientation} d3Scale={y} value={issue}>
                <TickLine
                  orient={
                    yAxisOrientation
                  } /*tickSize={-height + margin.top} style={{ stroke: '#bbb', strokeWidth: 0.5, strokeDashArray: '4, 2' }}*/
                />
                <TickText
                  orient={yAxisOrientation}
                  value={issue}
                  fontSize="10"
                />
              </BandTick>
            )
          })}
        </g>
        <g transform={`translate(${margin.left}, ${margin.top})`}>
          {flattened.map(d => {
            // Should use larger bubbles to indicate impact instead of
            // overlapping circles.  Can do it by state count and by population.
            return (
              <g key={`datapoint-${d.state}-${d.category}`}>
                <circle
                  cx={x(d.value) + 0.5}
                  cy={y(d.category)}
                  r="3"
                  fill={d3.schemeTableau10[categories.indexOf(d.category)]}
                  strokeWidth="0"
                  fillOpacity="0.7"
                />
                <title>{`${d.state}: ${d.category}, ${d.status}`}</title>
              </g>
            )
          })}
        </g>
        <g
          transform={`translate(${margin.left},${height - margin.bottom})`}
          fill="none"
          fontSize="12"
          fontFamily="sans-serif"
          textAnchor="start"
        >
          {x.ticks().map(value => {
            return (
              <NumberTick orient={xAxisOrientation} d3Scale={x} value={value}>
                <TickLine orient={xAxisOrientation} strokeWidth="0.7" />
                <TickText
                  orient={xAxisOrientation}
                  value={value}
                  fontSize="8"
                  tickPadding={0}
                />
              </NumberTick>
            )
          })}
        </g>
      </svg>
    </div>
  )
}
