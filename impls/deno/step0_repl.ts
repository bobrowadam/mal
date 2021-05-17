const PROMPT = "user> "

async function read(): Promise<string> {
  const buf = new Uint8Array(1024);
  await Deno.stdout.write(new TextEncoder().encode(PROMPT));
  const n = <number>await Deno.stdin.read(buf);
  return new TextDecoder().decode(buf.subarray(0, n)).trim();
}

function evalString(s: string) {
  return s;
}

async function print(s: string): Promise<void> {
  await Deno.stdout.write(new TextEncoder().encode(s + "\n"));
}

async function rep() {
  print(evalString(await read()));
}

async function main(): Promise<void> {
  while (true) {
    await rep();
  }
}

main()
