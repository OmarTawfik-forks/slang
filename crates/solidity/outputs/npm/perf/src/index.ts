import { glob } from "glob";
import { Parser as SlangParser } from "@nomicfoundation/slang/parser";
import antlrParser from "@solidity-parser/parser";
import assert from "node:assert";
import b from "benny";
import fs from "node:fs/promises";
import path from "node:path";

const REPO_ROOT = process.env["REPO_ROOT"]!;

const SOLIDITY_VERSION = "0.8.20"; // language version used in '@openzeppelin/contracts'

await runSuite("Small", "utils/structs/BitMaps.sol");
await runSuite("Medium", "utils/Address.sol");
await runSuite("Large", "utils/Arrays.sol");
await runSuite("Huge", "utils/Packing.sol");
await runSuite("Multiple", "utils/structs/**/*.sol");
await runSuite("All", "**/*.sol");

async function runSuite(suiteName: string, filesPattern: string) {
  const sourcesDir = path.join(REPO_ROOT, "node_modules/@openzeppelin/contracts");

  const filePaths = await glob(filesPattern, { cwd: sourcesDir, absolute: true });

  const sources = await Promise.all(filePaths.map((filePath) => fs.readFile(filePath, "utf8")));

  const loc = sources.reduce((total, contract) => total + contract.split("\n").length, 0);

  const reportsDir = path.join(REPO_ROOT, "crates/solidity/outputs/npm/perf/src/target/reports");

  return b.suite(
    `${suiteName} (${filePaths.length} files, ${Intl.NumberFormat().format(loc)} LOC)`,

    b.add(`Slang`, () => {
      const slangParser = SlangParser.create(SOLIDITY_VERSION);

      return () => {
        for (const source of sources) {
          const output = slangParser.parseFileContents(source);
          if (!output.isValid()) {
            assert.deepStrictEqual(output.errors(), []);
          }
        }
      };
    }),

    b.add(`Antlr`, () => {
      return () => {
        for (const source of sources) {
          const output = antlrParser.parse(source);
          if (output.errors !== undefined) {
            assert.deepStrictEqual(output.errors, []);
          }
        }
      };
    }),

    b.cycle(),
    b.complete(),

    b.configure({
      minDisplayPrecision: 0,

      cases: {
        initCount: 1, // The default number of times to execute a test on a benchmark's first cycle.
        minSamples: 10, // The minimum sample size required to perform statistical analysis.

        delay: 0.005, // The delay between test cycles (secs).
        minTime: 0, // The time needed to reduce the percent uncertainty of measurement to 1% (secs).
        maxTime: 10, // The maximum time a benchmark is allowed to run before finishing (secs).
      },
    }),

    b.save({ folder: reportsDir, file: suiteName, format: "chart.html", details: true }),
    b.save({ folder: reportsDir, file: suiteName, format: "table.html", details: true }),
  );
}
