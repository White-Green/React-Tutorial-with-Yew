const HtmlWebpackPlugin = require('html-webpack-plugin')
const path = require('path')

module.exports = {
    entry: path.resolve(__dirname, "src/index.js"),
    resolve: {
        extensions: ['.js', '.wasm', '.css'],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./src/index.html"
        })
    ],
    output: {
        path: path.resolve(__dirname, "output"),
        filename: "bundle.js"
    },
    devServer: {
        inline: true,          // http:localhost:8080/webpack-dev-server/ではなくhttp:localhost:8080/でアクセスできるようになる。
        host: "0.0.0.0"         // ※ dockerのコンテナで立てたサーバが他のホストからアクセスできるように全てのネットワークインターフェースに接続
    },
    module: {
        rules: [
            {
                test: /\.css/,
                use: [
                    "style-loader",
                    {
                        loader: "css-loader",
                        options: { url: false }
                    }
                ]
            }
        ]
    }
}