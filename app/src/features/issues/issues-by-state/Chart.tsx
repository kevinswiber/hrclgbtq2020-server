import React, { useState } from "react";
import * as d3 from "d3";
import * as styles from "./Chart.module.css";
import { StateIssue } from "../../../definitions/types";
import { ValueAxis } from "./ValueAxis";
import { IssueAxis } from "./IssueAxis";
import { Tooltip, Bars } from "./Bars";

export const Chart = ({
  data,
}: {
  data: Array<StateIssue>;
}): React.ReactElement => {
  const [tooltip, setTooltip] = useState<Tooltip>({
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

  return (
    <div id="issues-by-state-chart">
      <div id="tooltip" className={styles.tooltip} style={tooltip.style}>
        {tooltip.text}
      </div>
      <svg viewBox={`0,0,${width},${height}`}>
        <Bars data={data} x={x} y={y} setTooltip={setTooltip} />
        <IssueAxis data={data} x={x} y={y} />
        <ValueAxis x={x} margin={margin} />
      </svg>
    </div>
  );
};
