import React from "react";
import { Scale } from "../../typings/d3";

export enum Orientation {
  TOP = 1,
  RIGHT,
  BOTTOM,
  LEFT,
}

function translateX(x: number) {
  return "translate(" + x + ",0)";
}

function translateY(y: number) {
  return "translate(0," + y + ")";
}

function number(scale: Scale) {
  return (d) => +scale(d);
}

function center(scale: Scale, offset: number) {
  offset = Math.max(0, scale.bandwidth() - offset * 2) / 2;
  if (scale.round()) offset = Math.round(offset);
  return (d) => +scale(d) + offset;
}

interface AxisDomainProps<TAttrs> {
  orient: number;
  scale: Scale;
  tickSize?: number;
  pathAttrs?: TAttrs;
}

export const AxisDomain = <TAttrs,>({
  orient,
  scale,
  tickSize,
  ...pathAttrs
}: AxisDomainProps<TAttrs>): JSX.Element => {
  tickSize = tickSize === null || tickSize === undefined ? 6 : tickSize;
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const k = orient === Orientation.TOP || orient === Orientation.LEFT ? -1 : 1;

  const range = scale.range(),
    range0 = +range[0] + offset,
    range1 = +range[range.length - 1] + offset;

  const pathAttrsDerived = {
    className: "domain",
    stroke: "currentColor",
    d: "",
    ...pathAttrs,
  };

  if (orient === Orientation.LEFT || orient === Orientation.RIGHT) {
    pathAttrsDerived.d =
      tickSize > 0
        ? "M" +
          k * tickSize +
          "," +
          range0 +
          "H" +
          offset +
          "V" +
          range1 +
          "H" +
          k * tickSize
        : "M" + offset + "," + range0 + "V" + range1;
  } else {
    pathAttrsDerived.d =
      tickSize > 0
        ? "M" +
          range0 +
          "," +
          k * tickSize +
          "V" +
          offset +
          "H" +
          range1 +
          "V" +
          k * tickSize
        : "M" + range0 + "," + offset + "H" + range1;
  }

  return <path {...pathAttrsDerived} />;
};

interface TickProps<TickAttrs> {
  orient: Orientation;
  scale: Scale;
  value: string | number;
  line: JSX.Element;
  text: JSX.Element;
  tickAttrs?: TickAttrs;
}

export const Tick = <TickAttrs,>({
  orient,
  scale,
  value,
  line,
  text,
  ...tickAttrs
}: TickProps<TickAttrs>): JSX.Element => {
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform =
    orient === Orientation.TOP || orient === Orientation.BOTTOM
      ? translateX
      : translateY;
  const position = (scale.bandwidth ? center : number)(scale.copy(), offset);

  return (
    <g
      className="tick"
      opacity="1"
      transform={transform(position(value) + offset)}
      {...tickAttrs}
    >
      {line}
      {text}
    </g>
  );
};

interface TickLineProps<TLineAttrs> {
  orient: Orientation;
  tickSize?: number;
  lineAttrs?: TLineAttrs;
}
export const TickLine = <TLineAttrs,>({
  orient,
  tickSize = 6,
  ...lineAttrs
}: TickLineProps<TLineAttrs>): JSX.Element => {
  const k = orient === Orientation.TOP || orient === Orientation.LEFT ? -1 : 1;
  const x =
    orient === Orientation.LEFT || orient === Orientation.RIGHT ? "x" : "y";

  const initialAttrs = {
    stroke: "currentColor",
  };
  initialAttrs[`${x}2`] = k * tickSize;

  const lineAttrsDerived = { ...initialAttrs, ...lineAttrs };

  return <line {...lineAttrsDerived} />;
};

interface TickTextProps<TTextAttrs> {
  orient: Orientation;
  scale: Scale;
  tickSize?: number;
  tickPadding?: number;
  tickFormat(number): string;
  ticks?: number[];
  value: string;
  textAttrs?: TTextAttrs;
}

export const TickText = <TTextAttrs,>({
  orient,
  tickSize = 6,
  tickPadding = 3,
  tickFormat,
  ticks = [],
  value,
  scale,
  ...textAttrs
}: TickTextProps<TTextAttrs>): JSX.Element => {
  const k = orient === Orientation.TOP || orient === Orientation.LEFT ? -1 : 1;
  const x =
    orient === Orientation.LEFT || orient === Orientation.RIGHT ? "x" : "y";

  const format =
    tickFormat == null
      ? scale.tickFormat
        ? scale.tickFormat(...ticks)
        : (x) => x
      : tickFormat;
  const spacing = Math.max(+tickSize, 0) + +tickPadding;

  const initialAttrs = {
    fill: "currentColor",
    dy:
      orient === Orientation.TOP
        ? "0em"
        : orient === Orientation.BOTTOM
        ? "0.71em"
        : "0.32em",
  };
  initialAttrs[x] = k * spacing;

  textAttrs = Object.assign(initialAttrs, textAttrs);

  return <text {...textAttrs}>{format(value)}</text>;
};
