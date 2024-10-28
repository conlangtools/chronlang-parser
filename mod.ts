import peggy from "peggy";
import type { Stmt } from "./ast/statements.ts";

type ParseResult =
  | { ok: true; statements: readonly Stmt[] }
  | { ok: false; error: peggy.GrammarError };

const parser = peggy.generate(Deno.readTextFileSync("./grammar.pegjs"));

export * as ast from "./ast/mod.ts";
export const GrammarError = peggy.GrammarError;

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
