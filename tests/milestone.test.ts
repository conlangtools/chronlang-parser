import { assert, assertEquals } from "jsr:@std/assert";
import { parse } from "../src/mod.ts";

Deno.test("Parse an instant milestone without a language", () => {
  const source = "source-name";
  const code = `
    @ 1000
  `;

  const expectedAST = [{
    kind: "milestone",
    time: {
      kind: "instant",
      time: 1000,
      span: {
        source,
        start: { offset: 7, line: 2, column: 7 },
        end: { offset: 11, line: 2, column: 11 },
      },
    },
    language: null,
  }] as const;

  const result = parse(code, source);
  assert(result.ok);
  assertEquals(result.statements, expectedAST);
});

Deno.test("Parse a range milestone without a language", () => {
  const source = "source-name";
  const code = `
    @ 1000..1400
  `;

  const expectedAST = [{
    kind: "milestone",
    time: {
      kind: "range",
      start: 1000,
      end: 1400,
      span: {
        source,
        start: { offset: 7, line: 2, column: 7 },
        end: { offset: 17, line: 2, column: 17 },
      },
    },
    language: null,
  }] as const;

  const result = parse(code, source);
  assert(result.ok);
  assertEquals(result.statements, expectedAST);
});

Deno.test("Parse an instant milestone with a language", () => {
  const source = "source-name";
  const code = `
    @ 1000, PAu
  `;

  const expectedAST = [{
    kind: "milestone",
    time: {
      kind: "instant",
      time: 1000,
      span: {
        source,
        start: { offset: 7, line: 2, column: 7 },
        end: { offset: 11, line: 2, column: 11 },
      },
    },
    language: [
      "PAu",
      {
        source,
        start: { offset: 13, line: 2, column: 13 },
        end: { offset: 16, line: 2, column: 16 },
      },
    ],
  }] as const;

  const result = parse(code, source);
  assert(result.ok);
  assertEquals(result.statements, expectedAST);
});

Deno.test("Parse a range milestone with a language", () => {
  const source = "source-name";
  const code = `
    @ 1000..1400, PAu
  `;

  const expectedAST = [{
    kind: "milestone",
    time: {
      kind: "range",
      start: 1000,
      end: 1400,
      span: {
        source,
        start: { offset: 7, line: 2, column: 7 },
        end: { offset: 17, line: 2, column: 17 },
      },
    },
    language: [
      "PAu",
      {
        source,
        start: { offset: 19, line: 2, column: 19 },
        end: { offset: 22, line: 2, column: 22 },
      },
    ],
  }] as const;

  const result = parse(code, source);
  assert(result.ok);
  assertEquals(result.statements, expectedAST);
});
