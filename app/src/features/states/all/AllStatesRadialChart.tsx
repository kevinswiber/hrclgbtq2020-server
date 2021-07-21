import React, { ReactElement } from "react";
import * as d3 from "d3";
import {
  State,
  StateIssue,
  StateEqualityIndex,
} from "../../../definitions/types";

export const AllStatesRadialChart = (
  props: StateEqualityIndex
): ReactElement => {
  const width = 1024;
  const height = width;
  const innerRadius = 210;
  const outerRadius = Math.min(width, height) / 2 - 25;
  const regionOrder = ["Northeast", "South", "West", "Midwest"];

  const issues: { [id: string]: string } = {};
  const policy_values = {};

  for (const issue of props.issues.edges.map(iss => iss.node)) {
    policy_values[issue.name] = issue.states.map(s => Number(s.value)).sort();

    policy_values[issue.name] = [...new Set(policy_values[issue.name])];

    issues[issue.id] = issue.name;
  }

  const states = props.states.edges
    .map((s: { node: State }) => s.node)
    .sort((a: State, b: State) => {
      return (
        d3.ascending(
          regionOrder.indexOf(a.region),
          regionOrder.indexOf(b.region)
        ) || d3.ascending(a.id, b.id)
      );
    });

  const rotate = 4.5;
  const rotation = (rotate / states.length) * Math.PI * 2;

  const x = d3
    .scaleBand()
    .domain(states.map(d => d.id))
    .range([rotation, 2 * Math.PI + rotation]);

  interface ArcRegion {
    region: string;
    startAngle: number;
    endAngle: number;
    labelAngle: number;
  }

  const regions: ArcRegion[] = d3
    .groups(states, (d: State) => d.region)
    .map(([region, states]) => {
      const startAngle = x(states[0].id) || 0;
      const lastItem = states[states.length - 1];
      const endAngle = (x(lastItem.id) || 0) + x.bandwidth();
      const labelAngle =
        ((startAngle + endAngle) / 2 - Math.PI / 2) % (Math.PI * 2);
      return {
        region,
        startAngle,
        endAngle,
        labelAngle,
      };
    });

  const categoryOrder = [
    "TRANSGENDER_HEALTHCARE",
    "SCHOOL_ANTI_BULLYING",
    "PUBLIC_ACCOMMODATIONS",
    "MARRIAGE_EQUALITY",
    "HOUSING",
    "HATE_CRIMES",
    "GENDER_MARKER_UPDATES_ON_IDENTIFICATION",
    "EMPLOYMENT",
    "EDUCATION",
    "DISCRIMINATION_IN_CHILD_WELFARE",
    "ANTI_CONVERSION_THERAPY",
  ].map(n => issues[n]);

  const categories = categoryOrder.slice(0);

  interface Flattened {
    state: string;
    abbreviation: string;
    region: string;
    category: string;
    status: string;
    value: string | number;
  }
  const flattened: Flattened[] = states.flatMap((row: State) => {
    return categories.map(category => {
      const issue = row.issues.find((iss: StateIssue) => iss.name === category);
      return {
        state: row.name,
        abbreviation: row.id,
        region: row.region,
        category,
        status: issue ? issue.policy : "",
        value: issue ? issue.value : "",
      };
    });
  });

  const y = d3
    .scaleRadial()
    .domain([0, categories.length])
    .range([innerRadius, outerRadius]);

  const arc = d3
    .arc<Flattened>()
    .innerRadius(d => y(categoryOrder.indexOf(d.category)))
    .outerRadius(d => y(categoryOrder.indexOf(d.category) + 1))
    .startAngle(d => x(d.abbreviation) as number)
    .endAngle(d => (x(d.abbreviation) || 0) + x.bandwidth())
    .padRadius(innerRadius);

  const color = d3
    .scaleOrdinal()
    .domain(categories)
    .range(d3.schemeCategory10)
    .unknown("rgb(234,234,234)");

  const labelArc = d3
    .arc<ArcRegion>()
    .innerRadius(innerRadius - 20)
    .outerRadius(innerRadius - 20)
    .startAngle(d => d.startAngle)
    .endAngle(d => d.endAngle);

  const labelX = (angle: number, radius: number) => Math.cos(angle) * radius;
  const labelY = (angle: number, radius: number) => Math.sin(angle) * radius;

  const shouldFlip = (angle: number) => angle > 0 && angle < Math.PI;

  return (
    <div id="radial-chart">
      <svg
        style={{
          width: "100%",
          height: "auto",
          font: "14px sans-serif",
          padding: "20px",
        }}
        viewBox={`${-width / 2}, ${-height / 2}, ${width}, ${height}`}
        xmlns="http://www.w3.org/2000/svg"
        xmlnsXlink="http://www.w3.org/1999/xlink"
      >
        <g key="datapoints">
          {flattened.map(d => {
            return (
              <g key={`datapoint-${d.state}-${d.category}`}>
                <path
                  d={arc(d) as string}
                  stroke="white"
                  fill={color(d.category) as string}
                  fillOpacity={
                    ((policy_values[d.category].indexOf(d.value) + 1) /
                      policy_values[d.category].length) *
                    0.9
                  }
                />
                <title>{`${d.state}: ${d.category}, ${d.status}`}</title>
              </g>
            );
          })}
        </g>
        <g key="dividers">
          {regions.map(d => {
            return (
              <g
                key={`divider-${d.region}`}
                transform={`rotate(${(d.startAngle * 180) / Math.PI - 90})`}
              >
                <line
                  x1={innerRadius}
                  x2={outerRadius + 30}
                  stroke="black"
                  strokeWidth="5"
                />
              </g>
            );
          })}
        </g>
        <g key="region-label-paths">
          {regions.map(d => {
            return (
              <path
                key={`label-path-${d.region}`}
                d={labelArc(d) as string}
                id={`sei-chart-${d.region}`}
                stroke="none"
              />
            );
          })}
          <g
            key="region-label-texts"
            style={{ textTransform: "uppercase" }}
            fill="#888"
            textAnchor="middle"
          >
            {regions.map(d => {
              return (
                <text
                  key={`label-text-${d.region}`}
                  transform={
                    shouldFlip(d.labelAngle)
                      ? `translate(${labelX(d.labelAngle, 11)}, ${labelY(
                          d.labelAngle,
                          11
                        )})`
                      : undefined
                  }
                >
                  <textPath
                    xlinkHref={`#sei-chart-${d.region}`}
                    startOffset={shouldFlip(d.labelAngle) ? "75%" : "25%"}
                  >
                    {d.region}
                  </textPath>
                </text>
              );
            })}
          </g>
        </g>
        <g key="state-labels">
          {states.map(d => {
            return (
              <text
                key={`state-label-${d.id}`}
                textAnchor="middle"
                fill="#888"
                x={labelX(
                  (x(d.id) || 0) + x.bandwidth() / 2 - Math.PI / 2,
                  outerRadius + 16
                )}
                y={labelY(
                  (x(d.id) || 0) + x.bandwidth() / 2 - Math.PI / 2,
                  outerRadius + 16
                )}
                dy="0.31em"
              >
                {d.id}
              </text>
            );
          })}
        </g>
        <g key="legend">
          {categories.map((d, i) => {
            return (
              <g
                key={`legend-${i}`}
                transform={`translate(-120,${
                  (i - (categories.length - 1) / 2) * 20
                })`}
              >
                <rect width="18" height="18" fill={color(d) as string} />
                <text
                  style={{ font: "11px sans-serif" }}
                  x="24"
                  y="9"
                  dy="0.35em"
                >
                  {d}
                </text>
              </g>
            );
          })}
        </g>
      </svg>
    </div>
  );
};
