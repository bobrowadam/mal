import { MalAtom, MalData, MalList, Token } from './Types.ts';

const TOKENS_REGEX = /[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)/g;

export function readStr(s: string): MalData {
  const tokens = tokenize(s);
  const reader = new Reader(tokens);
  return readForm(reader);
}

function tokenize(s: string): Token[] {
  return s.match(TOKENS_REGEX)!.map(t => ({ Token: t })).slice(0, -1);
}

function readForm(reader: Reader): MalData {
  const currentToken = reader.peek();
  if (currentToken?.Token.includes('(')) {
    const list = readList(reader);
    return list
  }
  return readAtom(reader);
}

function readList(reader: Reader): MalList {
  const malList = { MalList: [] as MalData[] };
  let currentT = reader.next();
  while (currentT) {
    if (currentT?.Token.includes(')')) {
      return malList
    };
    const malData = readForm(reader);
    malList.MalList.push(malData);
    currentT = reader.peek();
  }
  throw new Error('Reached EOF in list');
}

function readAtom(reader: Reader): MalAtom {
  const currentToken = reader.next();
  switch (typeof currentToken?.Token) {
    case 'string':
      return { MalString: currentToken.Token }
    case 'undefined':
      return { MalNil: 'Nil' }
  }
}

class Reader {
  constructor(ts: Token[]) {
    this.ts = ts;
    this.position = 0;
    this.lastToken = this.ts.length;
  }
  ts: Token[]
  position: number;
  lastToken: number;
  peek(): Token | undefined {
    return this.ts[this.position]!;
  }
  next(): Token | undefined {
    return this.ts[this.position++];
  }
}
