import antfu from "@antfu/eslint-config"

export default antfu({
  stylistic: {
    indent: 2,
    quotes: "double",
  },
  typescript: true,
  vue: true,
  jsonc: true,
  yaml: true,
  markdown: true,
  ignores: [
    "tauri/**",
  ],
  rules: {
    "node/prefer-global/process": "off",
    "vue/first-attribute-linebreak": "off",
    "vue/v-on-event-hyphenation": "off",
    "vue/attribute-hyphenation": "off",
    "array-bracket-spacing": "off",
    "unused-imports/no-unused-vars": "warn",
    "unused-imports/no-unused-imports": "warn",
    "one-var": "off",
    "no-console": "off",
    "jsdoc/check-param-names": "off",
    "curly": ["warn", "multi-or-nest"],
    "antfu/if-newline": "off",
    "antfu/curly": "off",
    "antfu/consistent-list-newline": "off",
    "brace-style": "off",
    "vue/html-closing-bracket-newline": "off",
    "vue/no-parsing-error": [
      "error",
      {
        "invalid-first-character-of-tag-name": false,
      },
    ],
  },
})
