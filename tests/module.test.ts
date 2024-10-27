/*
  Tests ensuring the parser can handle large inputs
  & sequences of many statements.
*/

import { assertEquals, assert } from "jsr:@std/assert";
import { parse } from "../mod.ts";
import * as path from "jsr:@std/path";

Deno.test("Parse a sequence of statements", () => {
  const source = `
    import * from @core/ipa
    import {
      a, e, i, o, u
    } from ./local/vowels
          
    series F = { i, e, ε, æ }

    class X encodes (Place Manner) {
      ℂ = velar trill,
      ℤ = labiodental lateralFricative,
    }

    lang OEng : Old English // This isn't part of the name
    lang AmEng < OEng : American English
    lang RP < OEng : Received Pronunciation

    @ 1000, OEng
    @ 1940, AmEng

    - water /ˈwæ.ter/ {
      noun. liquid that forms the seas, lakes, rivers, and rain
      verb. pour or sprinkle water over a plant or area
    }

    $ [C+alveolar+stop] > [+flap] / V_V : Alveolar stops lenite to flaps intervocallically
  `

  const result = parse(source, "source-name")
  assert(Array.isArray(result))
  assertEquals(result.length, 11)
})

Deno.test("Parse a module from a file", () => {
  const sourceName = path.join(Deno.cwd(), "tests", "example.lang")
  const source = Deno.readTextFileSync(sourceName);
  const result = parse(source, sourceName)
  assertEquals(result.length, 11)
})
