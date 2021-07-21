import React, { useState } from "react";
import * as d3 from "d3";
import {
  AxisDomain,
  BandTick,
  NumberTick,
  Orientation,
  TickLine,
  TickText,
} from "../d3/Axis";
import * as styles from "./IssuesByStateBarChart.module.css";
import { StateIssue } from "../../typings/types";

export const IssuesByStateBarChart = ({
  data,
}: {
  data: Array<StateIssue>;
}): React.ReactElement => {
  const [tooltip, setTooltip] = useState({
    style: {
      left: 0,
      top: 0,
      opacity: 0,
    },
    text: "",
  });

  const margin = { top: 30, right: 60, bottom: 10, left: 60 };
  const barHeight = 25;
  const height =
    Math.ceil((data.length + 0.1) * barHeight) + margin.top + margin.bottom;
  const width = 700;

  const x = d3
    .scaleLinear()
    .domain([-6, 6])
    .rangeRound([margin.left, width - margin.right]);

  const y = d3
    .scaleBand<number>()
    .domain(d3.range(data.length))
    .rangeRound([margin.top, height - margin.bottom])
    .padding(0.1);

  const yTickFormat = (i: number) => data[i].name;

  const xTickLabels = ["Anti-LGBTQ", "Pro-LGBTQ"];
  const xTickFormat = (i: number) => {
    return xTickLabels[
      x
        .domain()
        .map((d) => d.toString())
        .indexOf(i.toString())
    ];
  };

  const toggleTooltip = (
    text: string,
    options: { hide: boolean } = { hide: false }
  ) => {
    return () => {
      const opacity = options.hide ? 0 : 1;
      setTooltip((prevTooltip) => ({
        ...prevTooltip,
        style: { ...prevTooltip.style, opacity },
        text,
      }));
    };
  };

  const placeTooltip = (ev) => {
    const left = ev.clientX + 20;
    const top = ev.clientY;
    setTooltip((prevTooltip) => ({
      ...prevTooltip,
      style: { ...prevTooltip.style, left, top },
    }));
  };

  return (
    <div id="issues-by-state-chart">
      <div id="tooltip" className={styles.tooltip} style={tooltip.style}>
        {tooltip.text}
      </div>
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
                  className={styles.bar}
                  style={{ width, transitionDelay: `${i * 20}ms` }}
                  onMouseOver={toggleTooltip(d.policy)}
                  onMouseMove={placeTooltip}
                  onMouseOut={toggleTooltip(d.policy, { hide: true })}
                  fill={d3.schemeSet1[d.value >= 0 ? 1 : 0]}
                  x={x(Math.min(d.value, 0))}
                  y={y(i)}
                  width={width}
                  height={y.bandwidth()}
                />
                {/*<title>{d.policy}</title>*/}
              </g>
            );
          })}
        </g>

        <g
          transform={`translate(${x(0)},0)`}
          fill="none"
          fontSize="12"
          fontFamily="sans-serif"
          textAnchor="end"
        >
          <AxisDomain
            orient={Orientation.LEFT}
            tickSize={0}
            range={y.range()}
          />
          {y.domain().map((d, i) => {
            const orient = Orientation.LEFT;
            const textAttrs: {
              textAnchor?: string;
              x?: number;
              className: string;
            } = {
              className: styles.issue,
            };
            if (data[i].value <= 0) {
              textAttrs.textAnchor = "start";
              textAttrs.x = 6;
            }

            return (
              <BandTick
                orient={orient}
                d3Scale={y}
                value={d}
                key={data[i].kind}
                line={<TickLine orient={orient} tickSize={0} />}
                text={
                  <TickText
                    orient={orient}
                    tickSize={0}
                    tickPadding={5}
                    value={d}
                    tickFormat={yTickFormat}
                    {...textAttrs}
                  />
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
          textAnchor="middle"
        >
          {x.domain().map((d) => {
            const orient = Orientation.TOP;
            return (
              <NumberTick
                orient={orient}
                d3Scale={x}
                value={d}
                key={`x-axis-${d}`}
                line={<TickLine orient={orient} />}
                text={
                  <TickText
                    orient={orient}
                    value={d}
                    tickFormat={xTickFormat}
                  />
                }
              />
            );
          })}
        </g>
      </svg>
    </div>
  );
};
