const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "production",
    entry: {
        index: "./bootstrap.js"
    },
    output: {
        path: dist,
        filename: "index.js",
        webassemblyModuleFilename: "index.wasm",
    },
    devServer: {
        contentBase: dist,
        historyApiFallback: true
    },
    plugins: [
        new CopyPlugin([
            path.resolve(__dirname, "static")
        ]),
        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
    ],
    module: {
        rules: [{
            test: /\.css$/,
            use: ['style-loader', 'css-loader']
        }, {
            test: /\.scss$/,
            use: ['style-loader', 'css-loader', 'sass-loader']
        }],
    }
};