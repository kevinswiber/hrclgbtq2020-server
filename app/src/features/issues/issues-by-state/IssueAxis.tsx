import React, { ReactElement } from "react";
import {
  AxisDomain,
  BandTick,
  Orientation,
  TickLine,
  TickText,
} from "../../../common/d3/Axis";
import * as styles from "./IssuesByStateBarChart.module.css";
import { StateIssue } from "../../../definitions/types";

export interface IssueAxisProps {
  data: StateIssue[];
  x: d3.ScaleLinear<number, number, never>;
  y: d3.ScaleBand<number>;
}

export const IssueAxis = ({ data, x, y }: IssueAxisProps): ReactElement => {
  const yTickFormat = (i: number) => data[i].name;
  return (
    <g
      transform={`translate(${x(0)},0)`}
      fill="none"
      fontSize="12"
      fontFamily="sans-serif"
      textAnchor="end"
    >
      <AxisDomain orient={Orientation.LEFT} tickSize={0} range={y.range()} />
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
          <BandTick orient={orient} d3Scale={y} value={d} key={data[i].kind}>
            <TickLine orient={orient} tickSize={0} />
            <TickText
              orient={orient}
              tickSize={0}
              tickPadding={5}
              value={d}
              tickFormat={yTickFormat}
              {...textAttrs}
            />
          </BandTick>
        );
      })}
    </g>
  );
};
