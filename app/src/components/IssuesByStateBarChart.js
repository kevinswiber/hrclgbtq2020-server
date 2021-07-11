import React, { useState, useEffect } from 'react';
import * as d3 from 'd3';
import { Axis, AxisDomain, Tick, Orientation, TickLine, TickText } from './Axis';

export const IssuesByStateBarChart = (props) => {
  const [current, setCurrent] = useState(props.current || 'choose');
  const ref = React.createRef();

  useEffect(() => {
    /*d3.select('#issues-by-state-chart svg')
      .append('g')
      .call(d3.axisLeft(y)
        .tickFormat((i) => data[i].name)
        .tickSize(0)
        .tickPadding(5))
      .call((g) => g.selectAll('.tick')
        .attr('key', (i) => `${data[i].kind}`))
      .call((g) => g.selectAll('.tick text')
        .filter(i => data[i].value <= 0)
        .attr('text-anchor', 'start')
        .attr('x', 6));*/
    window.location.hash = `#${current}`;
  });

  const sorted = props.states.sort((a, b) => d3.ascending(a.name, b.name));
  const change = (event) => {
    setCurrent(event.target.value);
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

  if (current.toLowerCase() === 'choose') {
    return (
      <div id="issues-by-state-chart">
        {select}
      </div>
    );
  }

  const data = props.states.find((s) => s.id === current).issues;

  const margin = { top: 30, right: 60, bottom: 10, left: 60 };
  const barHeight = 25;
  const height = Math.ceil((data.length + 0.1) * barHeight) + margin.top + margin.bottom;
  const width = 900

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

  const xAxis = (g) => g
    .attr('transform', `translate(0,${margin.top})`)
    .call(d3.axisTop(x)
      .ticks(x.domain()[1] - x.domain()[0])
    ).call((g) => g.select('.domain').remove());

  const yTickFormat = (i) => data[i].name;
  return (
    <div id="issues-by-state-chart">
      {select}
      <svg viewBox={`0,0,${width},${height}`} ref={ref}>
        <g key="bars">
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

        <g
          transform={`translate(${x(0)},0)`}
          fill="none"
          fontSize="10"
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
                key={data[i].kind}
                line={
                  <TickLine orient={orient} />
                }
                text={
                  <TickText
                    orient={orient}
                    value={d}
                    scale={x}
                    ticks={[x.domain()[1] - x.domain()[0]]} />
                }
              />
            );
          })}
        </g>
      </svg>
    </div >
  );
};