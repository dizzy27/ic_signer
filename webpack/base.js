const webpack = require("webpack");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const {
  VueLoaderPlugin
} = require("vue-loader");
const ElementPlus = require('unplugin-element-plus/webpack').default;

const isDevelopment = process.env.NODE_ENV !== "production";
const isProd = !isDevelopment;

let webpackBase = {
  resolve: {
    modules: ["node_modules", "*"],
    extensions: [".ts", ".tsx", ".js", ".jsx", ".mjs", ".json", ".vue"],
    fallback: {
      "http": require.resolve("stream-http"),
      "https": require.resolve("https-browserify"),
      "crypto": require.resolve("crypto-browserify"),
      "url": require.resolve("url/")
    }
  },
  module: {
    noParse: /^(vue|vue-router|vuex|vuex-router-sync)$/,
    rules: [{
        test: /\.css$/,
        use: [
          isProd ? {
            loader: MiniCssExtractPlugin.loader,
            options: {
              publicPath: "../" //这个很重要，与url-loader 生成的图片路经一至，需要这里设置
            }
          } : {
            loader: "vue-style-loader",
          },
          {
            loader: "css-loader",
            options: {
              sourceMap: isProd,
              importLoaders: 1,
              esModule: false
            }
          },
          {
            loader: "postcss-loader",
            options: {
              sourceMap: isProd,
              postcssOptions: {
                plugins: [
                  ["autoprefixer"],
                ],
              },
            }
          }
        ]
      },
      {
        test: /\.less$/,
        use: [
          isProd ? {
            loader: MiniCssExtractPlugin.loader,
            options: {
              publicPath: "../"
            }
          } : "style-loader",
          {
            loader: "css-loader",
            options: {
              sourceMap: isProd,
              importLoaders: 1,
              esModule: false

            }
          },
          {
            loader: "postcss-loader",
            options: {
              sourceMap: isProd,
              postcssOptions: {
                plugins: [
                  ["autoprefixer"],
                ],
              },
            }
          },
          {
            loader: "less-loader",
            options: {
              sourceMap: isProd
            },
          },
        ]
      },
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: {
          loader: "ts-loader",
          options: {
            onlyCompileBundledFiles: true,
            transpileOnly: true,
            appendTsSuffixTo: [/\.vue$/],
          },
        },
      },
      {
        test: /\.vue$/,
        loader: "vue-loader",
      },
      {
        test: /\.mjs$/,
        resolve: {
          byDependency: {
            esm: {
              fullySpecified: false
            }
          }
        }
      },
      {
        test: /\.jsx?$/, // /\.m?jsx?$/,
        exclude: (file) => {
          if (/\.vue\.jsx?$/.test(file)) {
            return false
          }
          return /node_modules/.test(file)
        },
        use: ["babel-loader"],
      },
      {
        test: /\.(gif|png|jpe?g|svg|ico)$/i,
        dependency: {
          not: ["url"]
        },
        use: [{
          loader: "url-loader",
          options: {
            name: `assets/[name].[hash:5].[ext]`,
            limit: 200,
          }
        }]
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/, // 处理字体
        use: {
          loader: "file-loader",
          options: {
            name: `assets/[name].[hash:5].[ext]`,
          }
        }
      }
    ]
  },
  plugins: [
    new VueLoaderPlugin(),
    new webpack.DefinePlugin({
      // 定义环境和变量
      __VUE_OPTIONS_API__: true,
      __VUE_PROD_DEVTOOLS__: isProd
    }),
    ElementPlus(),
    // new HtmlWebpackPlugin({
    //   filename: `index.html`,
    //   template: config.root + "/src/index.html",
    //   title: "demo",
    //   prod: true,
    //   hash: true,
    //   minify: {
    //     removeAttributeQuotes: true,
    //     collapseWhitespace: true,
    //     html5: true,
    //     minifyCSS: true,
    //     removeComments: true,
    //     removeEmptyAttributes: true
    //   }
    // }),
  ]
}

module.exports = webpackBase;