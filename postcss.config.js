const tailwindcss = require('tailwindcss');
const purgecss = require('@fullhuman/postcss-purgecss');
const cssnano = require('cssnano');
const autoprefixer = require('autoprefixer');
const postcssImport = require('postcss-import');
const postcssNested = require('postcss-nested');

module.exports = {
  plugins: [
    postcssImport(),
    tailwindcss(),
    postcssNested(),
    cssnano({
      preset: 'default',
    }),
    purgecss({
      content: [ 'templates/**/*.html' ],
      defaultExtractor: content => content.match(/[A-z0-9-:/]+/g),
    }),
    autoprefixer
  ]
};
