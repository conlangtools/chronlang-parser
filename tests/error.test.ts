import { assert } from "jsr:@std/assert";
import { parse } from "../src/mod.ts";

Deno.test("A syntax error does not throw", () => {
  const source = "source-name";
  const code = `
    import *
  `;
  const res = parse(code, source)
  assert(!res.ok)
})