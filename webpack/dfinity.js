const path = require("path");
let HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const CopyPlugin = require("copy-webpack-plugin");

function initCanisterEnv() {
    let localCanisters, prodCanisters;
    try {
        localCanisters = require(path.resolve(
            ".dfx",
            "local",
            "canister_ids.json"
        ));
    } catch (error) {
        console.log("No local canister_ids.json found. Continuing production");
    }
    try {
        prodCanisters = require(path.resolve("canister_ids.json"));
    } catch (error) {
        console.log("No production canister_ids.json found. Continuing with local");
    }

    const network =
        process.env.DFX_NETWORK ||
        (process.env.NODE_ENV === "production" ? "ic" : "local");

    const canisterConfig = network === "local" ? localCanisters : prodCanisters;

    return Object.entries(canisterConfig).reduce((prev, current) => {
        const [canisterName, canisterDetails] = current;
        prev[canisterName.toUpperCase() + "_CANISTER_ID"] =
            canisterDetails[network];
        return prev;
    }, {});
}
const canisterEnvVariables = initCanisterEnv();

const isDevelopment = process.env.NODE_ENV !== "production";

const frontendDirectory = "signer_assets";

const asset_entry = path.join("src", frontendDirectory, "index.html");

// II_FETCH_ROOT_KEY=1 II_DUMMY_CAPTCHA=1 dfx deploy --no-wallet --argument '(null)'
const LOCAL_II_CANISTER = "http://rwlgt-iiaaa-aaaaa-aaaaa-cai.localhost:8000/#authorize";

module.exports = {
    target: "web",
    mode: isDevelopment ? "development" : "production",
    entry: {
        // The frontend.entrypoint points to the HTML file for this build, so we need
        // to replace the extension to `.js`.
        index: path.join(__dirname, "../", asset_entry).replace(/\.html$/, ".js"),
    },
    devtool: isDevelopment ? "source-map" : false,
    resolve: {
        fallback: {
            assert: require.resolve("assert/"),
            buffer: require.resolve("buffer/"),
            events: require.resolve("events/"),
            stream: require.resolve("stream-browserify/"),
            util: require.resolve("util/"),
        }
    },
    output: {
        filename: "index.js",
        path: path.join(__dirname, "../", "dist", frontendDirectory),
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, "../", asset_entry),
            cache: false,
        }),
        new CopyPlugin({
            patterns: [{
                from: path.join(__dirname, "../", "src", frontendDirectory, "assets"),
                to: path.join(__dirname, "../", "dist", frontendDirectory),
            }, ],
        }),
        new webpack.EnvironmentPlugin({
          NODE_ENV: "development",
          LOCAL_II_CANISTER,
          ...canisterEnvVariables,
        }),
        new webpack.ProvidePlugin({
          Buffer: [require.resolve("buffer/"), "Buffer"],
          process: require.resolve("process/browser"),
        }),
    ],
    // proxy /api to port 8000 during development
    devServer: {
        proxy: {
            "/api": {
                target: "http://localhost:8000",
                changeOrigin: true,
                pathRewrite: {
                    "^/api": "/api",
                },
            },
        },
        hot: true,
        watchFiles: [path.resolve(__dirname, "../", "src", frontendDirectory)],
        liveReload: true,
    },
}