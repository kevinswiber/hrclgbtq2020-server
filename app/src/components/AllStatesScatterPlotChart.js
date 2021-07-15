import React from 'react';
import * as d3 from 'd3';
import { AxisDomain, Orientation, Tick, TickLine, TickText } from './d3/Axis';

export const AllStatesScatterPlotChart = (props) => {
  const width = 400;
  const height = width;
  const margin = { top: 30, right: 60, bottom: 10, left: 60 };

  let issues = {};
  let policy_values = {};

  for (const issue of props.issues) {
    policy_values[issue.node.name] = issue.node.states
      .map((s) => Number(s.value))
      .sort();

    policy_values[issue.node.name] = [...new Set(policy_values[issue.node.name])];

    issues[issue.node.id] = issue.node.name;
  }

  const states = props.states
    .map((s) => s.node)
    .sort((a, b) => d3.ascending(a.id, b.id));

  const categories = Object.keys(policy_values);

  const x = d3.scaleBand()
    .domain(categories)
    .range([margin.left, width - margin.right])
    .padding(1);

  const y = d3.scaleLinear()
    .domain([-6, 6])
    .range([height - margin.top, margin.bottom]);

  const flattened = states.flatMap(row => {
    return categories.map(category => {
      const issue = row.issues.find((iss) => iss.name === category);
      return {
        state: row.name,
        abbreviation: row.id,
        region: row.region,
        category,
        status: issue.policy,
        value: issue.value
      };
    })
  });

  return (
    <div id="all-states-scatter-plot">
      <svg viewBox={`0, 0, ${width}, ${height}`}>
        <g transform={`translate(0,${height - margin.top - margin.bottom})`}
          fill="none"
          fontSize="12"
          fontFamily="sans-serif"
          textAnchor="end">
          <AxisDomain orient={Orientation.BOTTOM} scale={x} tickSize="0" />
          {x.domain().map((issue) => {

            return (
              <Tick orient={Orientation.BOTTOM} scale={x} value={issue} tickSize="6"
                line={
                  <TickLine orient={Orientation.BOTTOM} tickSize={-height + margin.top} style={{ stroke: '#bbb', strokeWidth: 0.5, strokeDashArray: '4, 2' }} />
                }/*
                text={
                  <TickText orient={Orientation.BOTTOM} scale={x} value={issue} />
                }*/
              />
            )
          })}
        </g>
        <g>
          {flattened.map((d) => {
            return (
              <g key={`datapoint-${d.state}-${d.category}`}>
                <circle
                  cx={x(d.category)}
                  cy={y(d.value)}
                  r="3"
                  fill={d3.schemeCategory10[categories.indexOf(d.category)]}
                  strokeWidth="0"
                  fillOpacity="0.2" />
                <title>{`${d.state}: ${d.category}, ${d.status}`}</title>
              </g >
            );
          })}
        </g>
      </svg>
    </div >
  )
};