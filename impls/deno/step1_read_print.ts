import { printStr } from './printer.ts';
import { readStr } from './reader.ts';
import { MalData } from './Types.ts';

const PROMPT = "user> "

function read(s: string): MalData {
  return readStr(s);
}

function evalStr(s: MalData): MalData {
  return readStr(print(s));
}

function print(mal: MalData): string {
  return printStr(mal);
}

async function rep() {
  const buf = new Uint8Array(1024);
  // Print prompt
  await Deno.stdout.write(new TextEncoder().encode(PROMPT));

  // Get user input:
  const n = <number>await Deno.stdin.read(buf);
  const inputString = new TextDecoder().decode(buf.subarray(0, n)).trim();

  // Eval
  const evalMalData = evalStr(read(inputString));

  // Print back
  const stringifiedOutput = print(evalMalData); 
  await Deno.stdout.write(new TextEncoder().encode(stringifiedOutput + "\n"));
}

async function main(): Promise<void> {
  while (true) {
    await rep().catch(e => console.log(e))
  }
}

main()
