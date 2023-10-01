console.log("HECKINNNN", require("../package.json").version)
import './register'
import {createREPL, evaluate} from "./repl";
import * as path from 'node:path'
import * as vscode from 'vscode';

let myREPL = createREPL({name: 'test-repl-id'})
let chan = vscode.window.createOutputChannel("typescript-repl")

export function activate(context: vscode.ExtensionContext) {

  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  let disposable = vscode.commands.registerCommand('typescript-repl.evaluate', async () => {
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

