import * as d3 from 'd3';
import React, { useEffect, useState } from 'react';
import { graphql } from 'gatsby';
import { Container, FormControl, InputLabel, MenuItem, Select, makeStyles } from '@material-ui/core';
import { IssuesByStateBarChart } from '../components/IssuesByStateBarChart';

const useStyles = makeStyles((theme) => ({
  formControl: {
    margin: theme.spacing(1),
    minWidth: 120,
  },
  selectEmpty: {
    marginTop: theme.spacing(2),
  },
}));

const IssuesByStatePage = (props) => {
  useEffect(() => {
    window.location.hash = `#${current}`;
  });

  const classes = useStyles();

  const states = props.data.sei.states.edges.map((s) => s.node);
  const currentState = props.location.hash.length ?
    props.location.hash.slice(1).toUpperCase() :
    "";

  const [current, setCurrent] = useState(currentState);

  const sorted = states.sort((a, b) => d3.ascending(a.name, b.name));
  const change = (event) => {
    setCurrent(event.target.value);
  };

  const select = (
    <FormControl className={classes.formControl}>
      <InputLabel shrink htmlFor="state-select" id="state-label">State</InputLabel>
      <Select native inputProps={{ name: 'state', id: 'state-select' }} onChange={change} value={current}>
        <option key="none" value="">None</option>
        {sorted.map((d) => <option key={d.id} value={d.id}>{d.name}</option>)}
      </Select>
    </FormControl>
  );

  const data = (current && current !== "") ?
    states.find((s) => s.id === current).issues :
    null;

  return (
    <Container maxWidth="lg">
      <h1>State Equality Index 2020 - All States</h1>
      {select}
      {data &&
        <IssuesByStateBarChart data={data} />}
    </Container>
  )
};

export default IssuesByStatePage;

export const query = graphql`
query IssuesByStateQuery {
  sei {
    states {
      edges {
        node {
          id
          name
          region
          district
          score {
            kind
            description
          }
          issues {
            kind
            name
            policy
            value
          }
        }
      }
    }
  }
}
`;