const grammar = Deno.readTextFileSync("./grammar.pegjs");

Deno.writeTextFileSync("grammar.ts", `\
const grammar = String.raw\`\
${grammar}
\`

export default grammar;
`);
