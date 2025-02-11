import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import antlrParser from "@solidity-parser/parser";
import fs from "node:fs/promises";
import path from "node:path";
import { glob } from "glob";

test("using the parser", async () => {
  const contractsDir = path.join(process.env["REPO_ROOT"]!, "node_modules/@openzeppelin/contracts");
  const contractFiles = await glob("**/*.sol", { cwd: contractsDir, absolute: true });
  const contracts = await Promise.all(contractFiles.map((file) => fs.readFile(file, "utf8")));

  console.log(`
    Testing ${contracts.length} contracts from ${contractsDir}
  `);

  const slangParser = Parser.create("0.8.20");

  const parsers = [
    [
      "slang",
      () => {
        for (const contract of contracts) {
          const output = slangParser.parseFileContents(contract);
          assert(output.isValid());
        }
      },
    ],
    [
      "antlr",
      () => {
        for (const contract of contracts) {
          const output = antlrParser.parse(contract);
          assert(output.errors === undefined || output.errors.length === 0);
        }
      },
    ],
  ] as const;

  const attemptsCount = 25;

  for (const [name, parser] of parsers) {
    let min = Number.MAX_SAFE_INTEGER;
    let max = Number.MIN_SAFE_INTEGER;
    let total = 0;

    for (let i = 0; i < attemptsCount; i++) {
      const start = performance.now();
      parser();
      const end = performance.now();

      const current = end - start;
      min = Math.min(min, current);
      max = Math.max(max, current);
      total += current;
    }

    const average = total / attemptsCount;

    console.log(`
      Running ${name} parser ${attemptsCount} times...

      Min:   ${Intl.NumberFormat().format(min).padStart(15)} ms
      Max:   ${Intl.NumberFormat().format(max).padStart(15)} ms
      Avg:   ${Intl.NumberFormat().format(average).padStart(15)} ms
      Total: ${Intl.NumberFormat().format(total).padStart(15)} ms
    `);
  }
});
