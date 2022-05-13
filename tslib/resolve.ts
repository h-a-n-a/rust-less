const {resolveFile} = require('/Users/zhushijie/Desktop/github/rspack/crates/node_binding');

function resolve(filepath: string, import_path: string) {
    const res = resolveFile(filepath, import_path);
    console.log(res);
    return res;
}

export {resolve}