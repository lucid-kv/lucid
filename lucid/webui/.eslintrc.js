/*
rigwild's personal ESLint configuration
Using modern JavaScript syntax, very restrictive.
Preferably use with autofix on save.
This config is made for a Babel + Vue.js projet (vue-cli).
https://github.com/rigwild
*/

// prettier-ignore
module.exports = {
  parserOptions: {
    parser: 'babel-eslint',
    ecmaVersion: 2019,
    sourceType: 'module'
  },

  root: true,
  env: {
    node: true,
    es6: true
  },

  extends: [
    'plugin:vue/recommended'
  ],

  rules: {
    // Possible Errors
    // The following rules point out areas where you might have made mistakes.
    'comma-dangle': 1,
    'no-cond-assign': 2,
    'no-constant-condition': 2,
    'no-control-regex': 2,
    'no-debugger': 2,
    'no-dupe-args': 2,
    'no-dupe-keys': 2,
    'no-duplicate-case': 2,
    'no-empty': 2,
    'no-ex-assign': 2,
    'no-extra-boolean-cast': 2,
    'no-extra-parens': 0,
    'no-extra-semi': 2,
    'no-func-assign': 2,
    'no-inner-declarations': 2,
    'no-invalid-regexp': 2,
    'no-irregular-whitespace': 2,
    'no-negated-in-lhs': 2,
    'no-obj-calls': 2,
    'no-regex-spaces': 2,
    'no-sparse-arrays': 2,
    'no-unreachable': 2,
    'use-isnan': 2,
    'valid-jsdoc': 2,
    'valid-typeof': 2,

    // Best Practices
    // These are rules designed to prevent you from making mistakes.
    'block-scoped-var': 0,
    'complexity': 0,
    'curly': 'off',
    'default-case': 2,
    'dot-notation': 2,
    'eqeqeq': 2,
    'guard-for-in': 2,
    'no-alert': 2,
    'no-caller': 2,
    'no-div-regex': 2,
    'no-else-return': 2,
    'no-eq-null': 2,
    'no-eval': 2,
    'no-extend-native': 2,
    'no-extra-bind': 2,
    'no-fallthrough': 2,
    'no-floating-decimal': 2,
    'no-implied-eval': 2,
    'no-iterator': 2,
    'no-labels': 2,
    'no-lone-blocks': 2,
    'no-loop-func': 2,
    'no-multi-spaces': 2,
    'no-multi-str': 2,
    'no-native-reassign': 2,
    'no-new': 2,
    'no-new-func': 2,
    'no-new-wrappers': 2,
    'no-octal': 2,
    'no-octal-escape': 2,
    'no-proto': 2,
    'no-redeclare': 2,
    'no-script-url': 2,
    'no-self-compare': 2,
    'no-sequences': 2,
    'no-throw-literal': 2,
    'no-void': 2,
    'no-warning-comments': [0, { terms: ['todo', 'fixme'], location: 'start' }],
    'no-with': 2,
    'radix': 2,
    'vars-on-top': 2,
    'wrap-iife': 2,
    'yoda': 2,

    // Strict Mode
    // These rules relate to using strict mode.
    'strict': 0,

    // Variables
    // These rules have to do with variable declarations.
    'no-catch-shadow': 2,
    'no-delete-var': 2,
    'no-label-var': 2,
    'no-shadow': 2,
    'no-shadow-restricted-names': 2,
    'no-undef': 2,
    'no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
    'no-use-before-define': 2,

    // Stylistic Issues
    // These rules are purely matters of style and are quite subjective.
    'indent': [1, 2],
    'brace-style': ['error', 'stroustrup'],
    'camelcase': 1,
    'comma-spacing': [1, { before: false, after: true }],
    'comma-style': [1, 'last'],
    'consistent-this': [1, '_this'],
    'eol-last': 1,
    'key-spacing': [1, { beforeColon: false, afterColon: true }],
    'new-cap': [1, { newIsCap: true, capIsNew: false }],
    'new-parens': 1,
    'newline-after-var': 0,
    'no-array-constructor': 1,
    'no-mixed-spaces-and-tabs': 1,
    'no-multiple-empty-lines': [1, { max: 2 }],
    'no-trailing-spaces': 1,
    'no-underscore-dangle': 1,
    'quote-props': [1, 'consistent'],
    'quotes': [1, 'single'],
    'semi': ['error', 'never'],
    'keyword-spacing': 'warn',
    'space-before-function-paren': [1, { anonymous: 'always', named: 'never' }],
    'space-in-parens': [1, 'never'],
    'spaced-comment': 'warn',

    // ECMAScript 6
    // These rules are only relevant to ES6 environments and are off by default.
    'no-var': 2,

    // Vue.js
    'vue/max-attributes-per-line': 0,
    'vue/attributes-order': 0,
    'vue/singleline-html-element-content-newline': 0,
    'vue/html-self-closing': [
      'error',
      {
        html: {
          void: 'any',
          normal: 'any',
          component: 'always'
        },
        svg: 'always',
        math: 'always'
      }
    ]
  }
}
