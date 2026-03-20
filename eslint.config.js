import pluginVue from 'eslint-plugin-vue'
import tseslint from 'typescript-eslint'

export default tseslint.config(
  // Ignore build outputs
  {
    ignores: ['dist/**', 'src-tauri/**', 'node_modules/**'],
  },

  // Vue + TypeScript files
  ...pluginVue.configs['flat/recommended'],
  ...tseslint.configs.recommended,

  {
    files: ['**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },

  // Relaxed rules that suit a small Tauri project
  {
    rules: {
      // TypeScript
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'warn',

      // Vue
      'vue/multi-word-component-names': 'off', // common for single-file apps
      'vue/require-default-prop': 'off',
      'vue/html-self-closing': ['warn', { html: { void: 'always' } }],
    },
  },
)
