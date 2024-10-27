import peggy from "peggy"

export * as ast from "./ast/mod.ts"

const parser = peggy.generate(Deno.readTextFileSync("./grammar.pegjs"))

export function parse(source: string, sourceName: string) {
  return parser.parse(source, { grammarSource: sourceName })
}
