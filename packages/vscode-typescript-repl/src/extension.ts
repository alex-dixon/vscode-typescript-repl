console.log("HECKINNNN")
import {addHook} from 'pirates';
// import * as swc from "@swc/core"
import * as ts from "typescript";

const source = "let x: string  = 'string'";


function matcher(filename) {
  // Here, you can inspect the filename to determine if it should be hooked or
  // not. Just return a truthy/falsey. Files in node_modules are automatically ignored,
  // unless otherwise specified in options (see below).

  console.log("matcher", filename)

  // TODO: Implement your logic here
  return filename.endsWith('.ts');
}

console.log("gonna add hook")
const revert = addHook(
    (code, filename) => {
      try {
        let result = ts.transpileModule(code, { fileName:filename,compilerOptions: { module: ts.ModuleKind.CommonJS }});

        // const result = swc.transformSync(code, {
        //   filename,
        //   sourceFileName: filename,
        //   module: {type:'commonjs'},
        //   jsc: {
        //     loose:true,
        //
        //     parser: {syntax: "typescript",},
        //     target: "es2019",
        //   }
        // })
        console.log('result', result)
        return result.outputText
      } catch
        (e) {
        console.log('whattt', e)
      }
    }
    ,
    {
      exts: ['.ts'], matcher
    }
  )
;

import {createREPL, evaluate} from "./repl";
import * as path from 'node:path'
import {transformSync, transformSyncRegular} from "swc-ts-repl-transpile"

// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

// "TypeError: mod.require is not a function
// at dynamicRequire (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:197747:18)
// at exports.install (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:198328:21)
// at installSourceMapSupport (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:199818:26)
// at createFromPreloadedConfig (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:199815:7)
// at create (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:199712:14)
// at Object.register2 (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:199700:19)
// at Object.<anonymous> (/Users/alexdixon/.vscode/extensions/alex dixon.vscode-typescript-repl-0.0.1/dist/index.js:201443:8)
// at u._compile (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/loader.js:4:1271)
// at Module._extensions..js (node:internal/modules/cjs/loader:1371:10)
// at Module.load (node:internal/modules/cjs/loader:1171:32)
// at Module._load (node:internal/modules/cjs/loader:1012:12)
// at f._load (node:electron/js2c/asar_bundle:2:13330)
// at c._load (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/workbench/api/node/extensionHostProcess.js:135:5630)
// at m._load (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/workbench/api/node/extensionHostProcess.js:132:29116)
// at D._load (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/workbench/api/node/extensionHostProcess.js:99:19764)
// at Module.require (node:internal/modules/cjs/loader:1195:19)
// at require (node:internal/modules/cjs/helpers:110:18)
// at Function.i [as __$__nodeRequire] (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/loader.js:5:98)
// at n.rb (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/workbench/api/node/extensionHostProcess.js:132:30295)
// at async Promise.all (index 0)"


let myREPL = createREPL({name: 'test-repl-id'})
let chan = vscode.window.createOutputChannel("typescript-repl")
// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed

export function activate(context: vscode.ExtensionContext) {

  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activated
  console.log('Congratulations, your extension "typescript-repl" is now active!');

  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  let disposable = vscode.commands.registerCommand('typescript-repl.evaluate', async () => {
    // The code you place here will be executed every time your command is executed
    // Display a message box to the user
    vscode.window.showInformationMessage('Hello World from typescript-repl!');

    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      // No open text editor
      console.log("no editor")
      vscode.window.showInformationMessage('no selected editor?');
      return;
    }
    // const repl = createREPL({name:'test-repl-id'})

    const selection = editor.selection;
    const text = editor.document.getText(selection);
    const currentlyOpenTabFilePath = vscode.window.activeTextEditor.document.fileName;
    const currentlyOpenTabFileName = path.basename(currentlyOpenTabFilePath);
    const currentlyOpenTabDirname = path.dirname(currentlyOpenTabFilePath);

    try {
      const result = await evaluate({
        code: text,
        filename: currentlyOpenTabFileName,
        replId: myREPL.id,
        __dirname: currentlyOpenTabDirname
      }, undefined)

      if (result.type === 'error') {
        vscode.window.showInformationMessage(result.text);
        chan.appendLine(result.text)
      } else if (result.type === 'print') {
        chan.appendLine(result.result)
        // TODO. This is annoying because it forces the user to look at the panel...It looks like it doesn't show up otherwise though
        chan.show(true)
        // vscode.window.showInformationMessage(result.result);
      } else {
        console.error(result)
        throw new Error("Unhandled result")
      }
    } catch (e) {
      console.error(e)
      vscode.window.showInformationMessage(e.toString());
    }
  });

  context.subscriptions.push(disposable);
  context.subscriptions.push(
    vscode.commands.registerCommand('typescript-repl.evaluate-file', async () => {
      // The code you place here will be executed every time your command is executed
      // Display a message box to the user
      vscode.window.showInformationMessage('Hello World from typescript-repl!');

      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        // No open text editor
        console.log("no editor")
        vscode.window.showInformationMessage('no selected editor?');
        return;
      }
      // const repl = createREPL({name:'test-repl-id'})

      const text = editor.document.getText()
      const currentlyOpenTabFilePath = vscode.window.activeTextEditor.document.fileName;
      const currentlyOpenTabFileName = path.basename(currentlyOpenTabFilePath);
      const currentlyOpenTabDirname = path.dirname(currentlyOpenTabFilePath);

      try {
        const result = await evaluate({
          code: text,
          filename: currentlyOpenTabFileName,
          replId: myREPL.id,
          __dirname: currentlyOpenTabDirname
        }, undefined)

        if (result.type === 'error') {
          vscode.window.showInformationMessage(result.text);
          chan.appendLine(result.text)
        } else if (result.type === 'print') {
          chan.appendLine(result.result)
          // TODO. This is annoying because it forces the user to look at the panel...It looks like it doesn't show up otherwise though
          chan.show(true)
          // vscode.window.showInformationMessage(result.result);
        } else {
          console.error(result)
          throw new Error("Unhandled result")
        }
      } catch (e) {
        console.error(e)
        vscode.window.showInformationMessage(e.toString());
      }
    })
  );


}

// This method is called when your extension is deactivated
export function deactivate() {
  console.log("deactivated")
  // chan.dispose()
}

