// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

import * as fs from 'fs';
import * as path from 'path';
import * as parser from '../../pkg/leo_wasm';
import * as yaml from 'js-yaml';
import {JSON_SCHEMA, CORE_SCHEMA, DEFAULT_SCHEMA} from 'js-yaml';

// Path to the parser tests folder
const TESTS_PATH: string = path.join(__dirname, '../../../tests/parser');

// Path to the test expectations for parser tests
const EXPECTATIONS_PATH: string = path.join(__dirname, '../../../tests/expectations/parser/parser');

// List of folders containing parser tests
const TEST_FOLDERS: string[] = fs.readdirSync(TESTS_PATH);

// Test expectations enum, maps to string values: Pass and Fail
enum Expectation {
    Pass = "Pass",
    Fail = "Fail"
}

interface TestHeader {
    expectation: Expectation,
    namespace: string
}

interface TestResult {
    filePath: string,
    expectation: Expectation,
    received: Expectation,
    error: string|null
}

// Main function for the tests 
(function runTests() {
    let results: TestResult[] = [];

    for (let folder of TEST_FOLDERS) {
        results.push(...walkDir(folder));
    }

    if (results.length !== 0) {
        results.forEach((result) => {
            console.log(
                'Test failed. Path: %s; Expected: %s; Received: %s', 
                result.filePath, 
                result.expectation, 
                result.received
            );
        });
        process.exit(1);
    } else {
        console.log('All tests successfully passed.');
        process.exit(0);
    }
})();


/**
 * Test specific file an compare its outputs to the test expectations.
 * 
 * @param filePath Path to the tested file
 * @param outFile  Contents of the expectation file of there is one
 * @returns {TestResult[]} Tests that failed execution, empty means success
 */
function test(filePath: string, outFile: string|null): TestResult[] {
    const text = fs.readFileSync(filePath).toString();

    // Process the test's contents by cutting off the header
    const header = text.slice(text.indexOf('/*') + 2, text.indexOf('*/'));
    const testBody = text.slice(text.indexOf('*/') + 2);

    const {expectation /* , namespace */} = readHeader(header);
    
    const mismatches: TestResult[] = [];
    const outputs: string[] = [];
    
    // Get the tests by splitting by 2 newlines and sanitize each sample.
    const samples = testBody
        .split('\n\n')
        .map((el) => el.trim())
        .filter((el) => el.length !== 0);

    // Go through each test and run WASM parse function. Check the results agains expectations
    // and collect outputs to later compare to saved .out expectations.
    for (const sample of samples) {
        try {
            outputs.push(JSON.parse(parser.parse(sample))); // Parser outputs JSON
            if (expectation === Expectation.Fail) { // If expectation was Fail and it passed
                mismatches.push({
                    filePath,
                    expectation: Expectation.Fail, 
                    received: Expectation.Pass,
                    error: null
                });
            } 
        } catch (error) {
            outputs.push(error.toString());
            if (expectation === Expectation.Pass) { // If expectation was Pass and it failed
                mismatches.push({
                    error,
                    filePath,
                    expectation: Expectation.Pass, 
                    received: Expectation.Fail, 
                });
            }
        }
    }

    // Todo: After AST spans are removed, figure out a way to canonicalize strings
    // and get them to the same format as serde's. For now comparing the outputs as
    // strings is impossible.
    //  
    // const formedOut = yaml.dump({
    //     expectation: expectation,
    //     namespace: namespace,
    //     outputs: outputs
    // }, {schema: DEFAULT_SCHEMA});

    return mismatches;
}

/**
 * Recursively go through each directory. If a file is met,
 * then run test function for this file.
 * 
 * @param fileOrDir 
 */
function walkDir(fileOrDir: string): TestResult[] {
    const currTarget = path.join(TESTS_PATH, fileOrDir);
    let results: TestResult[] = []; // collect test results from sub calls

    if (fs.lstatSync(currTarget).isDirectory()) {
        for (const entry of fs.readdirSync(currTarget)) {
            results.push(...walkDir(path.join(fileOrDir, entry)));
        }
    } else {
        const outFilePath = path.join(EXPECTATIONS_PATH, fileOrDir + '.out');
        const outFile = fs.existsSync(outFilePath) ? fs.readFileSync(outFilePath).toString() : null;
        
        return results.concat(test(currTarget, outFile));
    }

    return results;
}

/**
 * Read a test header yaml and transform it into an Object.
 * 
 * @param header 
 * @returns {TestHeader} 
 */
function readHeader(header: string): TestHeader {
    const parsed: any = yaml.load(header);

    if (!parsed || parsed.constructor !== Object) {
        throw "Unable to read test expectations: " + header;
    }

    return {
        expectation: parsed.namespace,
        namespace: parsed.expectation,
    };
}
