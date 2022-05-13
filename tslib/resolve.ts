const { resolveFile } = require('/Users/zhushijie/Desktop/github/rspack/crates/node_binding');

function resolve(filepath: string, import_path: string) {
  return resolveFile(filepath, import_path);
}

export { resolve }