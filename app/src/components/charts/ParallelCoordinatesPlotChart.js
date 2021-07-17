import * as d3 from 'd3';
import React from 'react';
import { AxisDomain, Orientation } from '../d3/Axis';

export const ParallelCoordinatesPlotChart = ({ issues }) => {
  const ys = {};
  for (const issue of issues) {
    const domain = [];
    for (const state of issue.states) {
      if (domain.indexOf(state.value) === -1) {
        domain.push(state.value)
      }
    }
    ys[issue.name] = domain;
  }

  for (const [_k, v] of Object.entries(ys)) {
    v.sort(d3.ascending);
  }

  const margin = { top: 30, right: 60, bottom: 10, left: 60 };
  const spacer = 100;
  const height = 900;
  const width = Object.keys(ys).length * spacer + margin.left + margin.right;

  const xDomain = Object.keys(ys);
  const x = d3.scaleBand()
    .domain(Object.keys(ys))
    .range(Object.keys(ys).length * spacer);

  const [keys, values] = Object.entries(ys);
  return (
    <div id="parallel-chart">
      <svg viewBox={`0,0,${width},${height}`}>
        <g
          transform={`translate(${x(0)},${margin.top})`}
          fill="none"
          fontSize="12"
          fontFamily="sans-serif"
          textAnchor="end">
        </g>
      </svg>
    </div >
  )
};