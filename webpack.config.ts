import path from "path";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
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
  ]
};
