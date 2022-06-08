const {
    merge
} = require('webpack-merge');
const webpackConfig = require("./webpack/prod.js");
const dfinity = require('./webpack/dfinity');

module.exports = merge(webpackConfig, dfinity);