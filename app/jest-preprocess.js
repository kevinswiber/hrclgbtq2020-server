// eslint-disable-next-line @typescript-eslint/no-var-requires
const babelJest = require('babel-jest');

const babelOptions = {
  presets: ['babel-preset-gatsby']
};

module.exports = babelJest.default.createTransformer(babelOptions);