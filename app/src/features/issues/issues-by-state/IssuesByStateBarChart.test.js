import React from 'react';
import TestRenderer from 'react-test-renderer';

import { IssuesByStateBarChart } from './charts/IssuesByStateBarChart';

describe('IssuesByStateBarChart', () => {
  it('renders correctly', () => {
    const data = [
      {
        kind: 'TEST_ISSUE',
        name: 'Test Issue',
        policy: 'A policy for a test issue',
        value: 3
      },
      {
        kind: 'TEST_AGAIN_ISSUE',
        name: 'Test Again Issue',
        policy: 'A policy for a test again issue',
        value: -2
      }
    ];
    const testRenderer = TestRenderer.create(<IssuesByStateBarChart data={data}/>);

    expect(testRenderer.toJSON()).toMatchSnapshot();
  });
});