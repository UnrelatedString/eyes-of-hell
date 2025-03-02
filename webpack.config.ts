import path from "path";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

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
    static: path.resolve(__dirname, 'dist'),
    client: {
      overlay: false,
    },
  },
  plugins: [
    new CopyPlugin({
      patterns: [path.resolve(__dirname, "web/static")]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
