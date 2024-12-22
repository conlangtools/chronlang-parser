const grammar = Deno.readTextFileSync("./grammar.pegjs");

Deno.writeTextFileSync("grammar.ts", `\
const grammar: string = String.raw\`\
${grammar}
\`

export default grammar;
`);
