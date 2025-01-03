import pluginJs from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

/** @type {import("eslint").Linter.Config[]} */
export default [
  {
    files: [
      /*  Plain: */ "**/*.{js,ts}",
      /*  ESM:   */ "**/*.{mjs,mts}",
      /*  CJS:   */ "**/*.{cjs,cts}",
      /*  JSX:   */ "**/*.{jsx,tsx}",
    ],
  },
  {
    ignores: [
      // Packages and Dependencies
      "**/.hermit/**",
      "**/node_modules/**",
      // Generated Artifacts
      "**/target/**",
      // Git Submodules
      "**/submodules/**",
    ],
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        projectService: {
          allowDefaultProject: ["eslint.config.mjs", "jest.config.ts"],
        },
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    linterOptions: {
      reportUnusedDisableDirectives: "error",
    },
  },

  pluginJs.configs.recommended,
  ...tseslint.configs.recommendedTypeChecked,

  {
    rules: {
      "@typescript-eslint/ban-ts-comment": "off",
    },
  },
];
