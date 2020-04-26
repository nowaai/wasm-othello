const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const docs = path.resolve(__dirname, "docs");

module.exports = {
  mode: "production",
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env']
          }
        }
      }
    ]
  },
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: docs,
    filename: "[name].js"
  },
  devServer: {
    contentBase: docs,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
