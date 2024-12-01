import assert from "node:assert";

import { CompilationBuilder } from "@nomicfoundation/slang/compilation";

describe("can resolve imports", () => {
  test("empty builder", async () => {
    const builder = createBuilder();

    assertFiles(builder, []);
  });

  test("single file without imports", async () => {
    const builder = createBuilder();

    await builder.addFile("foo.sol");

    assertFiles(builder, ["foo.sol"]);
  });

  test("one file and one import", async () => {
    const builder = createBuilder();

    await builder.addFile("bar.sol");

    assertFiles(builder, ["foo.sol", "bar.sol"]);
  });

  test("transitive imports", async () => {
    const builder = createBuilder();

    await builder.addFile("baz.sol");

    assertFiles(builder, ["foo.sol", "bar.sol", "baz.sol"]);
  });
});

function createBuilder(): CompilationBuilder {
  return CompilationBuilder.create({
    languageVersion: "0.8.0",

    readFile: async (fileId) => {
      switch (fileId) {
        case "foo.sol":
          return `
            contract Foo {}
          `;
        case "bar.sol":
          return `
            import {Foo} from "foo.sol";
          `;
        case "baz.sol":
          return `
            import {Bar} from "bar.sol";
          `;
        default:
          return undefined;
      }
    },

    resolveImport: async (sourceFileId, importPath) => {
      const importString = importPath.node.unparse();

      switch (sourceFileId) {
        case "foo.sol": {
          switch (importString) {
            default:
              return undefined;
          }
        }
        case "bar.sol": {
          switch (importString) {
            case `"foo.sol"`:
              return "foo.sol";
            default:
              return undefined;
          }
        }
        case "baz.sol": {
          switch (importString) {
            case `"bar.sol"`:
              return "bar.sol";
            default:
              return undefined;
          }
        }
        default:
          return undefined;
      }
    },
  });
}

function assertFiles(builder: CompilationBuilder, expected: string[]) {
  const actual = builder
    .build()
    .files()
    .map((file) => file.id);

  assert.deepEqual(actual.sort(), expected.sort());
}
