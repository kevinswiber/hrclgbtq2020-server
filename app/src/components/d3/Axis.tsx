import { NumberValue, ScaleBand, ScaleContinuousNumeric, ScalePoint } from "d3";
import React from "react";

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

function number<TRange, TOutput, TUnknown>(
  scale: ScaleContinuousNumeric<TRange, TOutput, TUnknown>
) {
  return (d: NumberValue) => +scale(d);
}

function center<TDomain>(
  scale: ScaleBand<TDomain> | ScalePoint<TDomain>,
  offset: number
) {
  offset = Math.max(0, scale.bandwidth() - offset * 2) / 2;
  if (scale.round()) offset = Math.round(offset);
  return (d: TDomain) => +(scale(d) || 0) + offset;
}

interface AxisDomainProps<TRange, TAttrs> {
  orient: number;
  range: TRange[];
  tickSize?: number;
  pathAttrs?: TAttrs;
}

export const AxisDomain = <TRange, TAttrs>({
  orient,
  range,
  tickSize,
  ...pathAttrs
}: AxisDomainProps<TRange, TAttrs>): JSX.Element => {
  tickSize = tickSize === null || tickSize === undefined ? 6 : tickSize;
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const k = orient === Orientation.TOP || orient === Orientation.LEFT ? -1 : 1;

  const range0 = +range[0] + offset,
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

interface BandTickProps<TDomain, TickAttrs> {
  orient: Orientation;
  scale: ScaleBand<TDomain> | ScalePoint<TDomain>;
  value: TDomain;
  line: JSX.Element;
  text: JSX.Element;
  tickAttrs?: TickAttrs;
}

export const BandTick = <TDomain, TickAttrs>({
  orient,
  scale,
  value,
  line,
  text,
  ...tickAttrs
}: BandTickProps<TDomain, TickAttrs>): JSX.Element => {
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform =
    orient === Orientation.TOP || orient === Orientation.BOTTOM
      ? translateX
      : translateY;
  const position = center(scale.copy(), offset);

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

interface NumberTickProps<TickAttrs> {
  orient: Orientation;
  scale: ScaleContinuousNumeric<unknown, unknown, unknown>;
  value: NumberValue;
  line: JSX.Element;
  text: JSX.Element;
  tickAttrs?: TickAttrs;
}

export const NumberTick = <TickAttrs,>({
  orient,
  scale,
  value,
  line,
  text,
  ...tickAttrs
}: NumberTickProps<TickAttrs>): JSX.Element => {
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform =
    orient === Orientation.TOP || orient === Orientation.BOTTOM
      ? translateX
      : translateY;
  const position = number(scale.copy());

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
  tickSize?: number;
  tickPadding?: number;
  tickFormat?(count?: number): string;
  value: number;
  textAttrs?: TTextAttrs;
}

// For tickFormat, library consumers should use scale.tickFormat(...ticks)
// if no custom format exists.
export const TickText = <TTextAttrs,>({
  orient,
  tickSize = 6,
  tickPadding = 3,
  tickFormat,
  value,
  ...textAttrs
}: TickTextProps<TTextAttrs>): JSX.Element => {
  const k = orient === Orientation.TOP || orient === Orientation.LEFT ? -1 : 1;
  const x =
    orient === Orientation.LEFT || orient === Orientation.RIGHT ? "x" : "y";

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

  const val = tickFormat ? tickFormat(value) : value;
  return <text {...textAttrs}>{val}</text>;
};
