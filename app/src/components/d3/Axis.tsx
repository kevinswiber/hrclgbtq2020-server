import {
  NumberValue,
  ScaleBand,
  ScaleContinuousNumeric,
  ScaleOrdinal,
  ScalePoint,
} from "d3";
import React, { ReactElement } from "react";

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
}: AxisDomainProps<TRange, TAttrs>): ReactElement => {
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
  d3Scale: ScaleBand<TDomain> | ScalePoint<TDomain>;
  value: TDomain;
  line: ReactElement;
  text: ReactElement;
  tickAttrs?: TickAttrs;
}

export const BandTick = <TDomain, TickAttrs>({
  orient,
  d3Scale,
  value,
  line,
  text,
  ...tickAttrs
}: BandTickProps<TDomain, TickAttrs> &
  React.SVGProps<SVGGElement>): ReactElement => {
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform =
    orient === Orientation.TOP || orient === Orientation.BOTTOM
      ? translateX
      : translateY;

  const scaleCopy = d3Scale.copy();
  let bandOffset = Math.max(0, scaleCopy.bandwidth() - offset * 2) / 2;
  if (scaleCopy.round()) bandOffset = Math.round(bandOffset);
  const position = (d: TDomain) => +(scaleCopy(d) || 0) + bandOffset;

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

interface OrdinalTickProps<TDomain, TRange, TUnknown, TickAttrs> {
  orient: Orientation;
  d3Scale: ScaleOrdinal<TDomain, TRange, TUnknown>;
  value: TDomain;
  line: ReactElement;
  text: ReactElement;
  tickAttrs?: TickAttrs;
}

export const OrdinalTick = <TDomain, TRange, TUnknown, TickAttrs>({
  orient,
  d3Scale,
  value,
  line,
  text,
  ...tickAttrs
}: OrdinalTickProps<TDomain, TRange, TUnknown, TickAttrs> &
  React.SVGProps<SVGGElement>): ReactElement => {
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform =
    orient === Orientation.TOP || orient === Orientation.BOTTOM
      ? translateX
      : translateY;
  const position = (d: TDomain) => +d3Scale(d);

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

interface NumberTickProps<TRange, TOutput, TUnknown, TickAttrs> {
  orient: Orientation;
  d3Scale: ScaleContinuousNumeric<TRange, TOutput, TUnknown>;
  value: NumberValue;
  line: ReactElement;
  text: ReactElement;
  tickAttrs?: TickAttrs;
}

export const NumberTick = <TRange, TOutput, TUnknown, TickAttrs>({
  orient,
  d3Scale,
  value,
  line,
  text,
  ...tickAttrs
}: NumberTickProps<TRange, TOutput, TUnknown, TickAttrs> &
  React.SVGProps<SVGGElement>): ReactElement => {
  const offset =
    typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform =
    orient === Orientation.TOP || orient === Orientation.BOTTOM
      ? translateX
      : translateY;
  const position = (d: NumberValue) => +d3Scale(d);

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

interface TickLineProps<TLineAttrs> extends React.SVGProps<SVGLineElement> {
  orient: Orientation;
  tickSize?: number;
  lineAttrs?: TLineAttrs;
}
export const TickLine = <TLineAttrs,>({
  orient,
  tickSize = 6,
  ...lineAttrs
}: TickLineProps<TLineAttrs>): ReactElement => {
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

interface TickTextProps<TDomain, TTextAttrs>
  extends React.SVGProps<SVGTextElement> {
  orient: Orientation;
  tickSize?: number;
  tickPadding?: number;
  tickFormat?(count?: TDomain): string;
  value: TDomain;
  textAttrs?: TTextAttrs;
}

// For tickFormat, library consumers should use scale.tickFormat(...ticks)
// if no custom format exists.
export const TickText = <TDomain, TTextAttrs>({
  orient,
  tickSize = 6,
  tickPadding = 3,
  tickFormat,
  value,
  ...textAttrs
}: TickTextProps<TDomain, TTextAttrs>): ReactElement => {
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
