import pluginVue from 'eslint-plugin-vue'
import tseslint from 'typescript-eslint'
import vueParser from 'vue-eslint-parser'
import eslintConfigPrettier from 'eslint-config-prettier'

export default tseslint.config(
  {
    ignores: ['dist/**', 'src-tauri/**', 'node_modules/**'],
  },

  // Vue files: explicit vue-eslint-parser as outer, ts parser for <script>
  ...pluginVue.configs['flat/recommended'],
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },

  // TypeScript-eslint recommended scoped to .ts only
  ...tseslint.configs.recommended.map(config => ({
    ...config,
    files: ['**/*.ts', '**/*.tsx'],
  })),

  // Register @typescript-eslint plugin globally for .vue files
  {
    plugins: { '@typescript-eslint': tseslint.plugin },
    rules: {
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'warn',
      'vue/multi-word-component-names': 'off',
      'vue/require-default-prop': 'off',
      'vue/max-attributes-per-line': 'off',
      'vue/html-self-closing': ['warn', { html: { void: 'always' } }],
      'vue/block-lang': ['error', { script: { lang: 'ts' } }],
    },
  },

  // Prettier last — disables conflicting ESLint formatting rules
  eslintConfigPrettier,
)
