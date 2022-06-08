const {
  merge
} = require('webpack-merge');
const webpackbase = require('./base.js');
const TerserPlugin = require('terser-webpack-plugin'); //压缩代码
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const isDevelopment = process.env.NODE_ENV !== "production";

const webpackProdConfig = {
  optimization: {
    minimize: !isDevelopment,
    minimizer: [
      new TerserPlugin({
        parallel: 6,
        terserOptions: {
          compress: {
            warnings: false,
            drop_console: true,
            drop_debugger: true,
          },
        },
      }),
      // new CssMinimizerPlugin(),
    ],
    // splitChunks: {
    //   minSize: 20000,
    //   maxAsyncRequests: 10,
    //   cacheGroups: {
    //     vendor: { // 抽离第三方插件
    //       test: /node_modules/, // 指定是node_modules下的第三方包
    //       chunks: 'initial',
    //       name: 'common', // 打包后的文件名，任意命名
    //       priority: 10 // 设置优先级，防止和自定义的公共代码提取时被覆盖，不进行打包
    //     },
    //     default: {
    //       minChunks: 2,
    //       priority: -20,
    //       reuseExistingChunk: true
    //     }
    //   }
    // }
  },
  plugins: [
    new MiniCssExtractPlugin({
      // filename: `css/[name].${_version}.css`,
      // chunkFilename: `css/build.[name].${_version}.css`
      filename: "[name].css",
      chunkFilename: "[id].css",
    }),
  ]
}

module.exports = merge(webpackbase, webpackProdConfig)