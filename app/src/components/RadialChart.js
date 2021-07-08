import React, { useEffect } from 'react';
import * as d3 from 'd3';

export const RadialChart = (props) => {
    useEffect(() => {
        const width = 1024;
        const height = width;
        const innerRadius = 210;
        const outerRadius = Math.min(width, height) / 2 - 25;
        const regionOrder = ['Northeast', 'South', 'West', 'Midwest'];

        let issues = {};
        let policy_values = {};

        for (const issue of props.issues) {
            policy_values[issue.node.name] = issue.node.states.map((s) => s.value)
            issues[issue.node.id] = issue.node.name;
        }

        const states = props.states
            .map((s) => s.node)
            .sort((a, b) => {
                return d3.ascending(regionOrder.indexOf(a.region), regionOrder.indexOf(b.region))
                    || d3.ascending(a.id, b.id);
            });

        const rotate = 4.5;
        const rotation = (rotate / states.length) * Math.PI * 2;

        const x = d3
            .scaleBand()
            .domain(states.map(d => d.id))
            .range([rotation, 2 * Math.PI + rotation]);

        const regions = d3
            .groups(states, (d) => d.region)
            .map(([region, states]) => {
                const startAngle = x(states[0].id);
                const endAngle = x(states[states.length - 1].id) + x.bandwidth();
                const labelAngle = ((startAngle + endAngle) / 2 - Math.PI / 2) % (Math.PI * 2);

                return {
                    region,
                    startAngle,
                    endAngle,
                    labelAngle
                };
            });

        const categoryOrder = ['TRANSGENDER_HEALTHCARE', 'SCHOOL_ANTI_BULLYING', 'PUBLIC_ACCOMMODATIONS',
            'MARRIAGE_EQUALITY', 'HOUSING', 'HATE_CRIMES', 'GENDER_MARKER_UPDATES_ON_IDENTIFICATION',
            'EMPLOYMENT', 'EDUCATION', 'DISCRIMINATION_IN_CHILD_WELFARE', 'ANTI_CONVERSION_THERAPY'].map((n) => issues[n]);

        const categories = categoryOrder.slice(0);

        const flattened = states.flatMap(row => {
            return categories.map(category => {
                const issue = row.issues.find((iss) => iss.name === category);
                return {
                    state: row.name,
                    abbreviation: row.id,
                    region: row.region,
                    category,
                    status: issue.policy,
                    value: issue.value
                };
            })
        });

        const y = d3
            .scaleRadial()
            .domain([0, categories.length])
            .range([innerRadius, outerRadius]);

        const arc = d3
            .arc()
            .innerRadius((d) => y(categoryOrder.indexOf(d.category)))
            .outerRadius((d) => y(categoryOrder.indexOf(d.category) + 1))
            .startAngle((d) => x(d.abbreviation))
            .endAngle((d) => x(d.abbreviation) + x.bandwidth())
            .padRadius(innerRadius);

        const color = d3
            .scaleOrdinal()
            .domain(categories)
            .range(d3.schemeCategory10)
            .unknown('rgb(234,234,234)');

        const legend = g => g.append('g')
            .selectAll('g')
            .data(categories)
            .join('g')
            .attr('transform', (d, i) => `translate(-120,${(i - (categories.length - 1) / 2) * 20})`)
            .call(g => g.append('rect')
                .attr('width', 18)
                .attr('height', 18)
                .attr('fill', (d) => color(d)))
            .call(g => g.append('text')
                .style('font', '11px sans-serif')
                .attr('x', 24)
                .attr('y', 9)
                .attr('dy', '0.35em')
                .text(d => d));

        const labelX = (angle, radius) => Math.cos(angle) * radius;
        const labelY = (angle, radius) => Math.sin(angle) * radius;

        const dividers = (g) =>
            g
                .selectAll("g")
                .data(regions)
                .join("g")
                .attr("transform", d => `rotate(${(d.startAngle * 180) / Math.PI - 90})`)
                .append("line")
                .attr("x1", innerRadius)
                .attr("x2", outerRadius + 30)
                .attr("stroke", "black")
                .attr("stroke-width", 5);

        const labelArc = d3
            .arc()
            .innerRadius(innerRadius - 20)
            .outerRadius(innerRadius - 20)
            .startAngle(d => d.startAngle)
            .endAngle(d => d.endAngle);

        const regionLabels = (g) => {
            const shouldFlip = (angle) => angle > 0 && angle < Math.PI;
            g.selectAll('path')
                .data(regions)
                .join('path')
                .attr('d', labelArc)
                .attr('id', (d) => `sei-chart-${d.region}`)
                .attr('stroke', 'none');

            g.append('g')
                .style('text-transform', 'uppercase')
                .attr('fill', '#888')
                .attr('text-anchor', 'middle')
                .selectAll('text')
                .data(regions)
                .join('text')
                .attr('transform', d =>
                    shouldFlip(d.labelAngle)
                        ? `translate(${labelX(d.labelAngle, 11)}, ${labelY(d.labelAngle, 11)})`
                        : null
                )
                .append('textPath')
                .attr('xlink:href', d => `#sei-chart-${d.region}`)
                .attr('startOffset', d => (shouldFlip(d.labelAngle) ? '75%' : '25%'))
                .text((d) => d.region);
        };

        const stateLabels = (g) =>
            g
                .attr('text-anchor', 'middle')
                .attr('fill', '#888')
                .selectAll('text')
                .data(states)
                .join('text')
                .attr('x', d =>
                    labelX(x(d.id) + x.bandwidth() / 2 - Math.PI / 2, outerRadius + 16)
                )
                .attr('y', d =>
                    labelY(x(d.id) + x.bandwidth() / 2 - Math.PI / 2, outerRadius + 16)
                )
                .attr('dy', '0.31em')
                .text(d => d.id);

        const svg = d3
            .select('#chart')
            .append('svg')
            .attr('viewBox', [-width / 2, -height / 2, width, height])
            .style('width', '100%')
            .style('height', 'auto')
            .style('font', '14px sans-serif')
            .style('padding', '20px');

        const g = svg
            .append('g')
            .selectAll('g')
            .data(flattened)
            .join('g');

        g
            .append('path')
            .attr('d', arc)
            .attr('stroke', 'white')
            .attr('fill', (d) => color(d.category))
            .attr('fill-opacity', (d) => (policy_values[d.category].indexOf(d.value) + 1) * .2);

        g.append('title').text((d) => `${d.state}: ${d.category}, ${d.status} `);

        svg.append('g').call(dividers);
        svg.append('g').call(regionLabels);
        svg.append('g').call(stateLabels);
        svg.append('g').call(legend);
    });

    return (
        <div id="chart" />
    )
};