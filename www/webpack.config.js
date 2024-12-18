const path = require('path');
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: './bootstrap.js',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        filename: 'bootstrap.js',
        path: path.resolve(__dirname, 'public'),
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin({
            patterns: [{ from: "./index.html", to: "./" }],
        }),
    ],
};