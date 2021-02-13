// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
    mount: {
        src: "/",
    },
    plugins: ["@snowpack/plugin-typescript", ["@snowpack/plugin-optimize"]],
    packageOptions: {},
    devOptions: {},
    // bit of a hack to make sure the snowpack metadata isn't served in /build
    buildOptions: { clean: true, metaUrlPath: "../node_modules" },
    exclude: ["tests/*", "*.json", "*.lock", "snowpack.config.js"],
};
