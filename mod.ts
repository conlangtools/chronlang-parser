import peggy from "peggy";

export * as ast from "./ast/mod.ts";

import { Stmt } from "./ast/statements.ts";

type ParseResult =
  | { ok: true; statements: readonly Stmt[] }
  | { ok: false; error: peggy.GrammarError };

const parser = peggy.generate(Deno.readTextFileSync("./grammar.pegjs"));

export function parse(source: string, sourceName: string): ParseResult {
  try {
    const statements = parser.parse(source, { grammarSource: sourceName });
    return { ok: true, statements };
  } catch (error) {
    if (error instanceof peggy.GrammarError) {
      return { ok: false, error };
    } else {
      throw error;
    }
  }
}
