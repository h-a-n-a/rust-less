import { existsSync } from 'fs';

import type Less from 'less';


export default class LessAliasesPlugin {
  
  constructor() {
  }

  install(less: typeof Less, pluginManager: any) {

    class AliasPlugin extends less.FileManager {
      loadFile(
        filename: string,
        currentDirectory: string,
        options: Record<string, unknown>,
        enviroment: Less.Environment
      ) {
        let resolved;
    
        // try {
        //   resolved = config.node_resolve(
        //     filename,
        //     currentDirectory ? currentDirectory : stdinDir
        //   );
        //   resolved = getFilePathWithPlatform(
        //     resolved,
        //     config?.style?.platform
        //   );
        // } catch (err: any) {
        //   /**
        //    * 这里如果把 error throw 出去，会导致 less 跑飞，具体原因还未详细研究，
        //    * 所以这里将错误传递到外面，然后返回空文件，让 less 的转换进行下去，不要卡死在这里。
        //    */
        //   onResolveError(err);
        //   return Promise.resolve({
        //     filename,
        //     contents: '',
        //   });
        // }

        return super.loadFile(
          filename,
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
