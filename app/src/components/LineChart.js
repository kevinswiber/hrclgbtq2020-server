import React from 'react';
import * as d3 from 'd3';

export const LineChart = (props) => {
  const states = props.states.map((s) => s.node);
  const flattened = states.flatMap((s) => {
    return s.issues.map((iss) => {
      return {
        abbreviation: s.id,
        category: iss.name,
        value: iss.value
      }
    });
  });

  const margin = { top: 10, right: 30, bottom: 30, left: 60 },
    width = 920 - margin.left - margin.right,
    height = 800 - margin.top - margin.bottom;

  const x = d3.scaleLinear()
    .domain(d3.extent(flattened, (d) => d.value))
    .range([0, width]);

  const y = d3.scaleBand()
    .domain(states.map((s) => s.id))
    .range([height, 0]);

  const lineGenerator = d3.line((d) => d.value, (d) => d.abbreviation);

  console.log(flattened[0]);
  console.log('line:', lineGenerator(flattened[0]));
  return (
    <div id="line-chart">
      <svg
        width={width + margin.left + margin.right}
        height={height + margin.top + margin.bottom}>
        <g transform={`translate(${margin.left},${margin.top})`}>
          <g transform={`translate(0,${height})`}
            ref={(node) => d3.select(node).call(d3.axisBottom(x))} />
          <g ref={(node) => d3.select(node).call(d3.axisLeft(y))} />
          {flattened.map((d) => {
            console.log('line:', d3.line().x(x(d.value)).y(y(d.abbreviation))(d));
            return (
              <path
                key={`path-${d.abbreviation}-${d.category}`}
                fill="none"
                stroke="steelblue"
                strokeWidth="1.5"
                d={lineGenerator(d)} />
            );
            // d3.line().x(x(d.value)).y(y(d.abbreviation))
          })}
        </g>
      </svg>
    </div>
  )
}
