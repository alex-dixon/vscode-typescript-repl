import * as assert from 'assert';

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
import * as vscode from 'vscode';
import * as myExtension from '../../extension';

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));


suite('Extension Test Suite', async () => {
	vscode.window.showInformationMessage('Start all tests.' + myExtension);
	await sleep(2000)

	test('Sample test', () => {
		console.log("soooooooooooooooooooo")
		assert.strictEqual(-1, [1, 2, 3].indexOf(5));
		assert.strictEqual(-1, [1, 2, 3].indexOf(0));
	});
});
