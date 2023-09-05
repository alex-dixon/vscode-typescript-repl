// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
console.log("HECKINNNN")
import * as vscode from 'vscode';

import {TransformOutput, transformSync} from "swc-ts-repl-transpile"
import {tsToJS} from "./transpile";
import {createREPL, evaluate} from "./repl";
import * as path from 'node:path'
import * as typescript from 'typescript'

console.log("typescript version", typescript.version)

import * as tsnode from 'ts-node'

tsnode.register({transpileOnly: true, compilerOptions: {moduleResolution: 'nodenext'}})
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
