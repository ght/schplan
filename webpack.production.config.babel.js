const merge = require('webpack-merge');
const common = require('./webpack.common.config.babel.js');

module.exports = merge(common, {
  mode: 'production'
});
