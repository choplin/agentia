import eslintPluginSvelte from "eslint-plugin-svelte";
import tsParser from "@typescript-eslint/parser";
import tsPlugin from "@typescript-eslint/eslint-plugin";
import svelteParser from "svelte-eslint-parser";

export default [
  // Ignore patterns
  {
    ignores: [
      "node_modules/**",
      ".svelte-kit/**",
      "build/**",
      "dist/**",
      "src-tauri/**",
      "*.config.js",
      "*.config.ts",
    ],
  },

  // Base configuration
  {
    files: ["**/*.{js,ts,svelte}"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
    },
  },

  // TypeScript configuration
  {
    files: ["**/*.ts"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      "@typescript-eslint": tsPlugin,
    },
    rules: {
      ...tsPlugin.configs.recommended.rules,
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
        },
      ],
      "@typescript-eslint/no-explicit-any": "off",
    },
  },

  // Svelte configuration
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tsParser,
      },
    },
    plugins: {
      svelte: eslintPluginSvelte,
    },
    rules: {
      ...eslintPluginSvelte.configs.recommended.rules,
      "svelte/no-unused-svelte-ignore": "error",
      "svelte/valid-compile": "error",
    },
  },
];
