import path from "path";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";

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
    static: path.resolve(__dirname, 'dist')
  },
  plugins: [
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
