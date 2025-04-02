import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";

export default [
  pluginJs.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  eslintPluginPrettierRecommended,
  {
    files: ["src/**/*.js", "src/**/*.ts"],
    languageOptions: {
      sourceType: "commonjs",
      globals: {
        ...globals.jest,
        ...globals.es2022,
        ...globals.node,
      },
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        project: "./tsconfig.json",
      },
    },
    rules: {
      "@typescript-eslint/no-extraneous-class": "off",
      "@typescript-eslint/explicit-function-return-type": "error",
    },
  },
  {
    files: ["**/*.cjs"],
    ignores: ["dist/*", "node_modules/*"],
    languageOptions: {
      sourceType: "commonjs",
      globals: {
        ...globals.jest,
        ...globals.es2022,
        ...globals.node,
      },
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        project: "./tsconfig.json",
      },
    },
    rules: {
      "@typescript-eslint/no-extraneous-class": "off",
      "@typescript-eslint/explicit-function-return-type": "error",
    },
  },
  {
    files: ["**/*.mjs"],
    ignores: ["dist/*", "node_modules/*"],
    languageOptions: {
      sourceType: "module",
      globals: {
        ...globals.jest,
        ...globals.es2022,
        ...globals.node,
      },
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        project: "./tsconfig.json",
      },
    },
    rules: {
      "@typescript-eslint/no-extraneous-class": "off",
      "@typescript-eslint/explicit-function-return-type": "error",
    },
  },
];
