import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";

export default tseslint.config(
  pluginJs.configs.recommended,
  tseslint.configs.strictTypeChecked,
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
      "@typescript-eslint/explicit-function-return-type": "off",
      "@typescript-eslint/restrict-plus-operands": "off",
      "@typescript-eslint/no-unsafe-argument": "off",
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
      "@typescript-eslint/explicit-function-return-type": "off",
      "@typescript-eslint/restrict-plus-operands": "off",
      "@typescript-eslint/no-unsafe-argument": "off",
    },
  },
);
