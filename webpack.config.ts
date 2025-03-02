import path from "path";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import { experiments } from "webpack";

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./web/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
    clean: true
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [path.resolve(__dirname, "static")]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

    new HtmlWebpackPlugin({
      title: "Eyes of Hell",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
