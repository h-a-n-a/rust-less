import {build} from "esbuild";

build({
    entryPoints: ["tslib/index.ts"],
    outfile: "dist/main.js",
    minify: false,
    platform: "node",
    target: "es2020",
    bundle: true,
    sourcemap: true,
}).catch(() => process.exit(1));