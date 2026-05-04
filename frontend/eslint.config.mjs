import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

/** @type {import('eslint').Linter.Config[]} */
const eslintConfig = [
  // Ignore non-source directories
  {
    ignores: [
      ".next/**",
      "out/**",
      "build/**",
      "coverage/**",
      "next-env.d.ts",
      "**/*.stories.*",
      "**/.storybook/**",
      "node_modules/**",
    ],
  },
  // eslint-config-next already bundles @typescript-eslint/parser + react rules
  ...compat.extends("next/core-web-vitals"),
  // Custom rule overrides
  {
    rules: {
      "no-unused-vars": [
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
      "no-console": "warn",
      // NOTE: Do NOT add @typescript-eslint/* rules here unless
      // @typescript-eslint/eslint-plugin is explicitly installed —
      // referencing rules from a missing plugin causes a hard build error.
    },
  },
];

export default eslintConfig;
