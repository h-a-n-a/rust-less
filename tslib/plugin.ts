import type Less from 'less';
import { resolve } from './resolve';
import path from 'path';


export default class LessAliasesPlugin {

  public current_dir: string;

  public callback_error: Function;

  constructor(current_dir: string, callback_error: Function) {
    this.callback_error = callback_error;
    this.current_dir = current_dir
  }

  install(less: typeof Less, pluginManager: any) {

    let { current_dir, callback_error } = this;

    class AliasPlugin extends less.FileManager {
      loadFile(
        filename: string,
        currentDirectory: string,
        options: Record<string, unknown>,
        enviroment: Less.Environment
      ) {

        let resolved = undefined;
        try {
          let base_file = currentDirectory ? currentDirectory : path.dirname(current_dir);
          console.log("filepath->", base_file);
          console.log("import_path->", filename);
          resolved = resolve(base_file, filename);
        } catch (err: any) {
          callback_error(err);
          return Promise.resolve({
            filename,
            contents: '',
          });
        }

        return super.loadFile(
          resolved ?? filename,
          currentDirectory,
          options,
          enviroment,
        );
      }
    }

    pluginManager.addFileManager(
      new AliasPlugin()
    );
  }
}
