const path = require('path');

const slugify = state => state.toLowerCase().replace(" ", "-").replace(",", "");

exports.createPages = async function ({ actions, graphql }) {
  const { data } = await graphql(`
    query {
      sei {
        states {
          edges {
            node {
              name
            }
          }
        }
      }
    }
  `);
  for (const state of data.sei.states.edges) {
    const slug = slugify(state.node.name);
    actions.createPage({
      path: `/issues/issues-by-state/${slug}/`,
      component: path.resolve("./src/pages/issues/issues-by-state/index.tsx"),
      context: { selected: slug },
    });
  }
};
