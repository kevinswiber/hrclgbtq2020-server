const path = require('path');

exports.createPages = async ({ graphql, actions, reporter}) => {
  /*
  const {createPage} = actions;
  const result = await graphql(`
    query allStateIds {
      sei {
        states {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  `);

  if (result.errors) {
    throw result.errors;
  }

  for (s of result.data.sei.states.edges) {
    const id = s.node.id;
    const filePath = `/issues-by-state/${id}`;
    createPage({
      path: filePath,
      component: path.resolve('./src/pages/issues-by-state/index.js'),
    });
    reporter.success(`created ${filePath}`);
  }
  */

};