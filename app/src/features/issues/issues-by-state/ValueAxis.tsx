import React, { ReactElement } from "react";
import {
  NumberTick,
  Orientation,
  TickLine,
  TickText,
} from "../../../common/d3/Axis";

export interface ValueAxisProps {
  x: d3.ScaleLinear<number, number, never>;
  margin: {
    top: number;
  };
}

export const ValueAxis = ({ x, margin }: ValueAxisProps): ReactElement => {
  const xTickLabels = ["Anti-LGBTQ", "Pro-LGBTQ"];
  const xTickFormat = (i: number) => {
    return xTickLabels[
      x
        .domain()
        .map(d => d.toString())
        .indexOf(i.toString())
    ];
  };
  return (
    <g
      transform={`translate(0, ${margin.top})`}
      fill="none"
      fontSize="10"
      fontFamily="sans-serif"
      textAnchor="middle"
    >
      {x.domain().map(d => {
        const orient = Orientation.TOP;
        return (
          <NumberTick orient={orient} d3Scale={x} value={d} key={`x-axis-${d}`}>
            <TickLine orient={orient} />
            <TickText orient={orient} value={d} tickFormat={xTickFormat} />
          </NumberTick>
        );
      })}
    </g>
  );
};
