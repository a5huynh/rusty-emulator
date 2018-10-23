const CleanWebpackPlugin = require('clean-webpack-plugin');
const CopyWebpackPlugin = require("copy-webpack-plugin");
const Path = require('path');
const Webpack = require('webpack');

module.exports = {
  devServer: {
    contentBase: Path.join(__dirname, 'dist'),
    hot: true,
    progress: true,
    port: 9000
  },
  devtool: "source-map",
  entry: {
    bootstrap: "./src/bootstrap.js",
  },
  output: {
    path: Path.resolve(__dirname, "dist"),
    chunkFilename: '[name].chunk.js',
    filename: "[name].js",
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".json", ".wasm"]
  },
  module: {
    rules: [
      { test: /\.wasm$/, type: "webassembly/experimental" },
      // All files with a '.ts' or '.tsx' extension will be handled
      // by 'awesome-typescript-loader'.
      { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
      // All output '.js' files will have any sourcemaps re-processed
      // by 'source-map-loader'.
      { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
      {
        test: /\.scss$/,
        use: [
            "style-loader", // creates style nodes from JS strings
            "css-loader", // translates CSS into CommonJS
            "sass-loader" // compiles Sass to CSS, using Node Sass by default
        ]
      }
    ]
  },
  plugins: [
    new CleanWebpackPlugin(['dist']),
    new CopyWebpackPlugin(['src/index.html']),
    new Webpack.HotModuleReplacementPlugin(),
  ],
};