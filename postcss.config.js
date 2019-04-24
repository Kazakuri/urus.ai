const tailwindcss = require('tailwindcss');
const purgecss = require('@fullhuman/postcss-purgecss');
const cssnano = require('cssnano');
const autoprefixer = require('autoprefixer');
const postcssImport = require('postcss-import');
const postcssNested = require('postcss-nested');

module.exports = {
  plugins: [
    postcssImport(),
    tailwindcss('./tailwind.js'),
    postcssNested(),
    cssnano({
      preset: 'default',
    }),
    purgecss({
      content: [ 'templates/**/*.html' ],
      extractors: [
        {
          extractor: class {
            static extract(content) {
              return content.match(/[A-Za-z0-9-_:\/]+/g) || [];
            }
          },
          extensions: ["html"]
        }
      ]
    }),
    autoprefixer
  ]
};
