console.log("HECKINNNN", require("../package.json").version)
import './register'
import {logger} from "./logger";
import {createREPL, evaluate, repls} from "./repl";
import * as path from 'node:path'
import * as vscode from 'vscode';
import { EventEmitter } from 'node:stream';

let myREPL = createREPL({name: 'test-repl-id'})
let chan = vscode.window.createOutputChannel("typescript-repl")
let filepathsChangedSinceLastEvaluation = new Set<string>()

export function activate(context: vscode.ExtensionContext) {

  vscode.workspace.onWillSaveTextDocument(e => {
    logger.debug("Will save text document", e)
  })
  vscode.workspace.onDidChangeTextDocument(e => {
    if (!e.contentChanges.length) {
      return
    }

    logger.debug("Change text document content", e)

    // Could be any file type.
    // The extension will try to support any file type,
    // but as far as we know dirty tracking is only relevant for js/ts i.e. files that can be required
    if (e.document.languageId === 'typescript' || e.document.languageId === 'javascript') {
      // Note: other identifiers are available under document.uri if ever needed.
      // The contents of the set should match what is used in the require cache, expected to be an absolute filepath
      filepathsChangedSinceLastEvaluation.add(e.document.fileName)
      // Note: it would be best to remove from the set when the file is in the set and this event tells us the file
      // is not dirty anymore. It's not clear whether isDirty is reliable atm
    }
  })

  let disposable = vscode.commands.registerCommand('typescript-repl.evaluate', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      // No open text editor
      console.log("no editor")
      vscode.window.showInformationMessage('no selected editor?');
      return;
    }

    const selection = editor.selection;
    const text = editor.document.getText(selection);
    const currentlyOpenTabFilePath = vscode.window.activeTextEditor.document.fileName;
    const currentlyOpenTabDirname = path.dirname(currentlyOpenTabFilePath);

    try {
      if (
        // when this namespace has been defined and some js/ts files have changed since the last evaluation of it
        repls.get(myREPL.id)?.namespaces[currentlyOpenTabFilePath] &&
        filepathsChangedSinceLastEvaluation.size
      ) {
        let toRefresh = [...filepathsChangedSinceLastEvaluation]
        Object.entries(repls.get(myREPL.id)?.namespaces).forEach(([k, v]) => {
          let req = v.context.require as NodeJS.Require
          if (!req) {
            logger.warn("Expected require to be defined on context",)
            return
          }

          // Force the module to be reloaded from disk.
          // Note: maybe it's faster to load from the file in memory via vscode
          // todo. verify this isn't required for require.children
          toRefresh.forEach(id => {
            logger.debug("Refreshing", id)
            delete req.cache[id]
          })

          // clear for next evaluation
          filepathsChangedSinceLastEvaluation.clear()

        })
      }
      
      chan.appendLine(text + " =>")

      const result = await evaluate({
        code: text,
        filename: currentlyOpenTabFilePath,
        replId: myREPL.id,
        __dirname: currentlyOpenTabDirname,
      }, {
        send: (topic:string, message:any)=> {
         logger.debug("Recieved", topic, message) 
        if (topic==='repl:output') {
          if (message.type==='error') {
            chan.appendLine(message.text)
          }
          if (message.type==='print') {
            // if (message.input) {
            //   chan.appendLine(message.input.code + " =>")
            // }
            chan.appendLine(message.result)
            chan.show(true)
          }
        }
      }})

      // if (result.type === 'error') {
      //   vscode.window.showInformationMessage(result.text);
      //   chan.appendLine(result.text)
      // } else if (result.type === 'print') {
      //   chan.appendLine(result.result)
      //   // TODO. This is annoying because it forces the user to look at the panel...It looks like it doesn't show up otherwise though
      //   chan.show(true)
      //   // vscode.window.showInformationMessage(result.result);
      // } else {
      //   console.error(result)
      //   throw new Error("Unhandled result")
      // }
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
        logger.error("no activeTextEditor")
        vscode.window.showInformationMessage('no selected editor?');
        return;
      }

      const text = editor.document.getText()
      const currentlyOpenTabFilePath = vscode.window.activeTextEditor.document.fileName;
      const currentlyOpenTabDirname = path.dirname(currentlyOpenTabFilePath);

      try {
        const result = await evaluate({
          code: text,
          filename: currentlyOpenTabFilePath,
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
          logger.error(result)
          throw new Error("Unhandled result")
        }
      } catch (e) {
        logger.error(e)
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

