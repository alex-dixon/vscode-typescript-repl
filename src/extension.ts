// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

import { TransformOutput, transformSync } from "swc-ts-repl-transpile"

export const tsToJS = (code: string): TransformOutput => {
  const result = transformSync(code)
  console.log("transform result", result)
  return result
}

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "typescript-repl" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	let disposable = vscode.commands.registerCommand('typescript-repl.helloWorld', () => {
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

		var selection = editor.selection;
		var text = editor.document.getText(selection);

		vscode.window.showInformationMessage(tsToJS(text).code);

	});

	context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {
	console.log("deactivated")
}
