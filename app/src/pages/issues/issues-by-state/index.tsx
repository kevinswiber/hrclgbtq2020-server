import * as d3 from "d3";
import React, { ReactElement, useEffect, useState } from "react";
import { PageProps, graphql, Link } from "gatsby";
import { BrowserHistory, createBrowserHistory } from "history";
import {
  Container,
  FormControl,
  InputLabel,
  Paper,
  Select,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  makeStyles,
} from "@material-ui/core";
import { Chart } from "../../../features/issues/issues-by-state/Chart";
import * as pageStyles from "./issues-by-state.module.css";
import { Data } from "../../../definitions/types";

const useStyles = makeStyles(theme => ({
  formControl: {
    margin: theme.spacing(1),
    minWidth: 120,
  },
  selectEmpty: {
    marginTop: theme.spacing(2),
  },
  tableContainer: {
    marginTop: 20,
    marginBottom: 60,
  },
  table: {
    minWidth: 300,
  },
  stateLinks: {
    marginTop: 40,
    marginBottom: 40,
  },
}));

const slugify = (state: string) =>
  state.toLowerCase().replace(" ", "-").replace(",", "");
const slugMap: { [slug: string]: string } = {};
const reverseSlugMap: { [stateName: string]: string } = {};

const IssuesByStatePage = (props: PageProps<Data>): ReactElement => {
  const [current, setCurrent] = useState<string>(
    (props.pageContext as { selected?: string }).selected || ""
  );
  const states = props.data.sei.states.edges.map(s => s.node);
  const classes = useStyles();

  let browserHistory: BrowserHistory;

  useEffect(() => {
    if (!browserHistory) {
      browserHistory = createBrowserHistory();
    }

    if (!current && current !== "") {
      const parts = props.location.pathname.split("/");
      const currentState = parts[parts.length - 2];

      setCurrent(currentState !== "issues-by-state" ? currentState : "");
    }
  });

  const change = (value: string) => {
    setCurrent(value);
    const endPath = value ? `${value}/` : "";
    browserHistory.push(`/issues/issues-by-state/${endPath}`, { state: value });
  };

  const sorted = states.sort((a, b) => d3.ascending(a.name, b.name));

  for (const s of sorted) {
    const slug = slugify(s.name);
    slugMap[slug] = s.name;
    reverseSlugMap[s.name] = slug;
  }

  const select = (
    <FormControl className={`${classes.formControl} ${pageStyles.noprint}`}>
      <InputLabel shrink htmlFor="state-select" id="state-label">
        State
      </InputLabel>
      <Select
        native
        inputProps={{ name: "state", id: "state-select" }}
        onChange={e => change(e.target.value as string)}
        value={current}
      >
        <option key="none" value="">
          None
        </option>
        {sorted.map(d => {
          return (
            <option key={d.id} value={reverseSlugMap[d.name]}>
              {d.name}
            </option>
          );
        })}
      </Select>
    </FormControl>
  );

  const stateLinks = (
    <Container className={classes.stateLinks}>
      {states.map((state, i) => {
        return (
          <>
            <Link
              key={state.id}
              to={`/issues/issues-by-state/${slugify(state.name)}/`}
            >
              {state.name}
            </Link>
            {i < states.length - 1 && <span> | </span>}
          </>
        );
      })}
    </Container>
  );

  const data =
    current && current !== ""
      ? states.find(s => s.name === slugMap[current])
      : null;

  return (
    <Container maxWidth="md">
      <h2>{(data && data.name) || ""} State policies for LGBTQ+ issues</h2>
      {select}
      {!data && stateLinks}
      {data && (
        <div>
          <Chart data={data.issues} />
          <TableContainer className={classes.tableContainer} component={Paper}>
            <Table className={classes.table} aria-label="state policy table">
              <TableHead>
                <TableRow>
                  <TableCell>Issue</TableCell>
                  <TableCell>Policy</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {data.issues.map(row => {
                  return (
                    <TableRow key={row.kind}>
                      <TableCell component="th" scope="row">
                        {row.name}
                      </TableCell>
                      <TableCell>{row.policy}</TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          </TableContainer>
          <Container className={classes.stateLinks}>
            {states.map((state, i) => {
              return (
                <>
                  {state.name !== data.name && (
                    <Link
                      key={state.id}
                      to={`/issues/issues-by-state/${slugify(state.name)}/`}
                    >
                      {state.name}
                    </Link>
                  )}
                  {state.name === data.name && state.name}
                  {i < states.length - 1 && <span> | </span>}
                </>
              );
            })}
          </Container>
        </div>
      )}
    </Container>
  );
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
