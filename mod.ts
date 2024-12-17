/**
 * @module
 * This module contains the reference parser for the Chronlang
 * conlang definition language.
 * 
 * ``
 */

import peggy from "npm:peggy@4.1";
import grammar from "./grammar.ts"
import type { Stmt } from "./ast/statements.ts";

const parser: peggy.Parser = peggy.generate(grammar);

export * as ast from "./ast/mod.ts";
export type SyntaxError = peggy.parser.SyntaxError;

type ParseResult =
  | { ok: true; statements: readonly Stmt[] }
  | { ok: false; error: peggy.parser.SyntaxError };

/**
 * Parse a string of Chronlang source code into a 
 * sequence of {@link Stmt}s
 */
export function parse(source: string, sourceName: string): ParseResult {
  try {
    const statements = parser.parse(source, { grammarSource: sourceName });
    return { ok: true, statements };
  } catch (error) {
    if (error instanceof peggy.parser.SyntaxError) {
      return { ok: false, error };
    } else {
      throw error;
    }
  }
}
