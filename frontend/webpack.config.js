const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require('path');

const ROOT = path.resolve( __dirname, 'src' );
const DESTINATION = path.resolve( __dirname, 'dist' );

module.exports = {
  entry: "./bootstrap.ts",
  output: {
    path: DESTINATION,
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../"),
      forceWatch: true
    }),
  ],
  module: {
    rules: [
      {
        include: [
          path.resolve(__dirname, "js")
        ],
      },
      {
        test: /\.ts$/,
        exclude: [ /node_modules/ ],
        use: 'awesome-typescript-loader'
      },
      {
        test: /\.wasm$/,
        type: "webassembly/experimental"
      },
      {
        test: /\.js$/,
        exclude: ["/node_modules/"],
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env'],
            plugins: ['@babel/plugin-syntax-dynamic-import']
          }
        }
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],
    modules: [
      ROOT,
      'node_modules'
    ]
  },
};
