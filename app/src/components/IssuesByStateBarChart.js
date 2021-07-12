import React, { useState } from 'react';
import * as d3 from 'd3';
import { AxisDomain, Tick, Orientation, TickLine, TickText } from './d3/Axis';
import { bar, issue } from './IssuesByStateBarChart.module.css'

export const IssuesByStateBarChart = ({ data }) => {
  const margin = { top: 30, right: 60, bottom: 10, left: 60 };
  const barHeight = 25;
  const height = Math.ceil((data.length + 0.1) * barHeight) + margin.top + margin.bottom;
  const width = 900

  const x = d3.scaleLinear()
    .domain([-6, 6])
    .rangeRound([margin.left, width - margin.right]);

  const y = d3.scaleBand()
    .domain(d3.range(data.length))
    .rangeRound([margin.top, height - margin.bottom])
    .padding(0.1);

  const yTickFormat = (i) => data[i].name;

  const xTickLabels = ['Worse', 'Better'];
  const xTickFormat = (i) => xTickLabels[x.domain().indexOf(i)];

  return (
    <div id="issues-by-state-chart">
      <svg viewBox={`0,0,${width},${height}`}>
        <g key="bars">
          {data.map((d, i) => {
            let width = Math.abs(x(d.value) - x(0));
            if (width === 0) {
              width = 2;
            }

            return (
              <g key={d.kind}>
                <rect
                  className={bar}
                  fill={d3.schemeSet1[d.value >= 0 ? 1 : 0]}
                  x={x(Math.min(d.value, 0))}
                  y={y(i)}
                  width={width}
                  style={{ width: `${width}` }}
                  height={y.bandwidth()} />
                <title>{d.policy}</title>
              </g>
            );
          })}
        </g>

        <g
          transform={`translate(${x(0)},0)`}
          fill="none"
          fontSize="12"
          fontFamily="sans-serif"
          textAnchor="end">
          <AxisDomain orient={Orientation.LEFT} tickSize="0" scale={y} />
          {y.domain().map((d, i) => {
            const orient = Orientation.LEFT;
            const textAttrs = {};
            if (data[i].value <= 0) {
              textAttrs.textAnchor = 'start';
              textAttrs.x = 6;
            }

            return (
              <Tick
                orient={orient} scale={y}
                value={d}
                key={data[i].kind}
                line={
                  <TickLine orient={orient} tickSize="0" />
                }
                text={
                  <TickText
                    orient={orient}
                    tickSize="0"
                    tickPadding="5"
                    className={issue}
                    value={d}
                    scale={y}
                    tickFormat={yTickFormat}
                    {...textAttrs} />
                }
              />
            );
          })}
        </g>

        <g
          transform={`translate(0, ${margin.top})`}
          fill="none"
          fontSize="10"
          fontFamily="sans-serif"
          textAnchor="middle">
          {x.domain().map((d, i) => {
            const orient = Orientation.TOP;
            return (
              <Tick
                orient={orient} scale={x}
                value={d}
                key={`x-axis-${d}`}
                line={
                  <TickLine orient={orient} />
                }
                text={
                  <TickText
                    orient={orient}
                    value={d}
                    scale={x}
                    tickFormat={xTickFormat} />
                }
              />
            );
          })}
        </g>
      </svg>
    </div >
  );
};