const {resolveFile} = require('../node_resolver/binding');

function resolve(filepath: string, import_path: string) {
    const res = resolveFile(filepath, import_path);
    return res;
}

export {resolve}