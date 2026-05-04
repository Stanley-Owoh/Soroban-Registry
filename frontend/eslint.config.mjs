import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const eslintConfig = [
  ...compat.extends("next/core-web-vitals", "next/typescript"),
  {
    ignores: [
      ".next/**",
      "out/**",
      "build/**",
      "coverage/**",
      "next-env.d.ts",
      "**/*.stories.*",
      "**/.storybook/**",
    ],
  },
  {
    rules: {
      // Unused vars are a cleanup task, not a build blocker.
      // Prefix with _ to intentionally suppress (e.g. _unusedParam).
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          vars: "all",
          args: "after-used",
          ignoreRestSiblings: true,
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
      "no-unused-vars": "off", // defer entirely to @typescript-eslint/no-unused-vars
      // aria-expanded on input[type=text] is a false positive in this codebase
      "jsx-a11y/role-supports-aria-props": "warn",
    },
  },
];

export default eslintConfig;
