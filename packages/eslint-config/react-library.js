/** @type {import("eslint").Linter.Config} */
module.exports = {
  extends: ["next", "turbo"],
  plugins: ["only-warn"],
  rules: {
    // We are using this rule to prevent using the `useEffect` hook.
    "no-restricted-imports": [
      "error",
      {
        name: "react",
        importNames: ["useEffect"],
        message: "Please use `useEffect` from `react-tracked` instead.",
      },
    ],
  },
  globals: {
    React: true,
    JSX: true,
  },
  env: {
    browser: true,
  },
};
