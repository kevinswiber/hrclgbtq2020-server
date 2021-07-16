exports.onCreateNode = ({ node }) => {
  console.log(`Node created of type "${node.internal.type}"`);
};