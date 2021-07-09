import { HashHistory } from '@reach/router';
import React from 'react';
import * as d3 from 'd3';

export class IssuesByStateBarChart extends React.Component {
  constructor(props) {
    super(props)
    this.state = { current: props.current || 'choose' };
  }

  componentDidUpdate() {
    window.location.hash = `#${this.state.current}`;
  }

  render() {
    const current = this.state.current;
    const sorted = this.props.states.sort((a, b) => d3.ascending(a.name, b.name));
    const change = (event) => {
      this.setState({ current: event.target.value })
    };

    const select = (
      <select onChange={change} value={current}>
        <option value="choose">-- Choose a state --</option>
        {sorted.map((d) => {
          return (
            <option key={d.id} value={d.id} >{d.name}</option>
          )
        })}
      </select>
    );

    if (this.state.current.toLowerCase() === 'choose') {
      return (
        <div id="issues-by-state-chart">
          {select}
        </div>
      );
    }

    const data = this.props.states.find((s) => s.id === this.state.current).issues;

    const margin = { top: 30, right: 60, bottom: 10, left: 60 };
    const barHeight = 25;
    const height = Math.ceil((data.length + 0.1) * barHeight) + margin.top + margin.bottom;
    const width = 1024;

    let xDomain = d3.extent(data, (d) => d.value);
    if (xDomain[0] > -2) {
      xDomain[0] = -2;
    }
    if (xDomain[1] < 2) {
      xDomain[1] = 2;
    }

    const x = d3.scaleLinear()
      .domain(xDomain)
      .rangeRound([margin.left, width - margin.right]);

    const y = d3.scaleBand()
      .domain(d3.range(data.length))
      .rangeRound([margin.top, height - margin.bottom])
      .padding(0.1);

    return (
      <div id="issues-by-state-chart">
        {select}
        <svg viewBox={`0,0,${width},${height}`}>
          <g>
            {data.map((d, i) => {
              let width = Math.abs(x(d.value) - x(0));
              if (width === 0) {
                width = 2;
              }

              return (
                <g key={d.kind}>
                  <rect
                    fill={d3.schemeSet1[d.value >= 0 ? 1 : 0]}
                    x={x(Math.min(d.value, 0))}
                    y={y(i)}
                    width={width}
                    height={y.bandwidth()} />
                  <title>{d.policy}</title>
                </g>
              );
            })}
          </g>
          {/*
        <g fontFamily="sans-serif" fontSize="10">
          {data.map((d, i) => {
            return (
              <text
                key={d.kind}
                textAnchor={d.value < 0 ? 'end' : 'start'}
                x={x(d.value) + Math.sign(d.value) * 4}
                y={y(i) + y.bandwidth() / 2}
                dx="0.35em"
                dy="0.35em">{d.value}</text>
            )
          })}
        </g>
        */}
          <g transform={`translate(${x(0)},0)`}
            ref={(node) => d3.select(node)
              .call(d3.axisLeft(y)
                .tickFormat((i) => data[i].name)
                .tickSize(0)
                .tickPadding(5))
              .call((g) => g.selectAll('.tick text')
                .filter(i => data[i].value < 0)
                .attr('text-anchor', 'start')
                .attr('x', 6))}>
          </g>
          <g transform={`translate(0,${margin.top})`}
            ref={(node) => d3.select(node)
              .call(d3.axisTop(x)
                .ticks(x.domain()[1] - x.domain()[0])
              )}>
          </g>
        </svg>
      </div>
    );
  }
};