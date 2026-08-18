import pluginJs from "@eslint/js";
import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";

export default defineConfig(eslintPluginPrettierRecommended, {
  files: ["**/*.{js,ts}"],
  extends: [pluginJs.configs.recommended, tseslint.configs.strictTypeChecked],
  rules: {
    "@typescript-eslint/explicit-function-return-type": "error",
  },
  languageOptions: {
    parserOptions: {
      projectService: true,
    },
  },
});
