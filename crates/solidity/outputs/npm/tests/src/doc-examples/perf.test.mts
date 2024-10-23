import { readRepoFile } from "../utils/files.mjs";
import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind } from "@nomicfoundation/slang/cst";
import antlrParser from "@solidity-parser/parser";

test("using the parser", async () => {
  const source = await readRepoFile(
    "crates/solidity/testing/perf/node_modules/@openzeppelin/contracts/utils/math/SafeCast.sol",
  );

  const slangParser = Parser.create("0.8.20");

  const parsers = [
    [
      "slang",
      () => {
        const output = slangParser.parse(NonterminalKind.SourceUnit, source);
        assert(output.isValid());
      },
    ],
    [
      "antlr",
      () => {
        const output = antlrParser.parse(source);
        assert(output.errors === undefined || output.errors.length === 0);
      },
    ],
  ] as const;

  const attemptsCount = 100;

  for (const [name, parser] of parsers) {
    console.log(`Running ${name} parser x${attemptsCount} times...`);

    const start = performance.now();

    let min = Number.MAX_SAFE_INTEGER;
    let max = Number.MIN_SAFE_INTEGER;

    for (let i = 0; i < attemptsCount; i++) {
      const localStart = performance.now();
      parser();
      const localEnd = performance.now();

      min = Math.min(min, localEnd - localStart);
      max = Math.max(max, localEnd - localStart);
    }

    const end = performance.now();

    console.log(`
      Min: ${min.toFixed(2)}ms
      Max: ${max.toFixed(2)}ms

      Avg: ${((end - start) / attemptsCount).toFixed(2)}ms
      Total: ${((end - start) / 1000).toFixed(2)}sec
    `);
  }
});
