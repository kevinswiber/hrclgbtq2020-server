export interface Data {
  sei: StateEqualityIndex;
}

export interface StateIssue {
  kind: string;
  name: string;
  policy: string;
  value: number;
}

export interface State {
  id: string;
  name: string;
  region: string;
  issues: Array<StateIssue>;
}

export interface IssueState {
  id: string;
  name: string;
  policy: string;
  value: number;
}

export interface Issue {
  id: string;
  name: string;
  states: Array<IssueState>;
}

export interface StateEqualityIndex {
  states: {
    edges: Array<{ node: State }>;
  };
  issues: {
    edges: Array<{ node: Issue }>;
  };
}
