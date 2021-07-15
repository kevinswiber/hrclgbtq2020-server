import React from 'react';

const top = 1,
  right = 2,
  bottom = 3,
  left = 4;

export const Orientation = Object.freeze({
  TOP: top,
  RIGHT: right,
  BOTTOM: bottom,
  LEFT: left
});

function translateX(x) {
  return "translate(" + x + ",0)";
}

function translateY(y) {
  return "translate(0," + y + ")";
}

function number(scale) {
  return d => +scale(d);
}

function center(scale, offset) {
  offset = Math.max(0, scale.bandwidth() - offset * 2) / 2;
  if (scale.round()) offset = Math.round(offset);
  return d => +scale(d) + offset;
}

export const AxisDomain = ({ orient, scale, tickSize, ...pathAttrs }) => {
  tickSize = (tickSize === null || tickSize === undefined) ? 6 : tickSize;
  const offset = typeof window !== 'undefined' && window.devicePixelRatio > 1 ? 0 : 0.5;
  const k = orient === top || orient === left ? -1 : 1;

  const range = scale.range(),
    range0 = +range[0] + offset,
    range1 = +range[range.length - 1] + offset;

  pathAttrs = Object.assign({
    className: 'domain',
    stroke: 'currentColor'
  }, pathAttrs);

  if (orient === left || orient === right) {
    pathAttrs.d = ((tickSize > 0) ? "M" + k * tickSize + "," + range0 + "H" + offset + "V" + range1 + "H" + k * tickSize : "M" + offset + "," + range0 + "V" + range1);
  } else {
    pathAttrs.d = ((tickSize > 0) ? "M" + range0 + "," + k * tickSize + "V" + offset + "H" + range1 + "V" + k * tickSize : "M" + range0 + "," + offset + "H" + range1);
  }

  return <path {...pathAttrs} />;

};

export const Tick = ({ orient, scale, value, line, text, ...tickAttrs }) => {
  const offset = typeof window !== 'undefined' && window.devicePixelRatio > 1 ? 0 : 0.5;
  const transform = orient === top || orient === bottom ? translateX : translateY;
  const position = (scale.bandwidth ? center : number)(scale.copy(), offset);
  console.log(value, 'position:' + position(value));

  return (
    <g
      className="tick"
      opacity="1"
      transform={transform(position(value) + offset)}
      {...tickAttrs}>
      {line}
      {text}
    </g>)
}

export const TickLine = ({ orient, tickSize = 6, ...lineAttrs }) => {
  const k = orient === top || orient === left ? -1 : 1;
  const x = orient === left || orient === right ? 'x' : 'y';

  const initialAttrs = {
    stroke: 'currentColor'
  };
  initialAttrs[`${x}2`] = k * tickSize

  lineAttrs = Object.assign(initialAttrs, lineAttrs);

  return (
    <line {...lineAttrs} />
  );
};

export const TickText = ({ orient, tickSize, tickPadding, tickFormat, ticks = [], value, scale, ...textAttrs }) => {
  tickPadding = tickPadding || 3;
  const tickSizeInner = tickSize || 6;
  const k = orient === top || orient === left ? -1 : 1;
  const x = orient === left || orient === right ? 'x' : 'y';

  const format = tickFormat == null ? (scale.tickFormat ? scale.tickFormat.apply(scale, ticks) : (x) => x) : tickFormat;
  const spacing = Math.max(tickSizeInner, 0) + tickPadding;

  const initialAttrs = {
    fill: 'currentColor',
    dy: orient === top ?
      '0em' :
      orient === bottom ?
        '0.71em' :
        '0.32em'
  };
  initialAttrs[x] = k * spacing;

  textAttrs = Object.assign(initialAttrs, textAttrs);

  return (
    <text {...textAttrs}>
      {format(value)}
    </text>
  );
}