import React, { Dispatch, SetStateAction } from "react";
import * as d3 from "d3";
import * as styles from "./Chart.module.css";
import { StateIssue } from "../../../definitions/types";

export interface Tooltip {
  style: {
    left: number;
    top: number;
    opacity: number;
  };
  text: string;
}

export interface BarsProps {
  data: StateIssue[];
  x: d3.ScaleLinear<number, number, never>;
  y: d3.ScaleBand<number>;
  setTooltip: Dispatch<SetStateAction<Tooltip>>;
}

export const Bars = ({
  data,
  x,
  y,
  setTooltip,
}: BarsProps): React.ReactElement => {
  const toggleTooltip = (
    text: string,
    options: { hide: boolean } = { hide: false }
  ) => {
    return () => {
      const opacity = options.hide ? 0 : 1;
      setTooltip(prevTooltip => ({
        ...prevTooltip,
        style: { ...prevTooltip.style, opacity },
        text,
      }));
    };
  };

  const placeTooltip = ev => {
    const left = ev.clientX + 20;
    const top = ev.clientY;
    setTooltip(prevTooltip => ({
      ...prevTooltip,
      style: { ...prevTooltip.style, left, top },
    }));
  };

  return (
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
          </g>
        );
      })}
    </g>
  );
};
