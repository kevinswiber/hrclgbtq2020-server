module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  plugins: ["@typescript-eslint", "react"],
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react/recommended",
    "standard",
  ],
  env: {
    browser: true,
    es6: true,
    jest: true,
    node: true,
  },
};
