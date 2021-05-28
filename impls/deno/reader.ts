import { MalAtom, MalData, MalList, Token } from './Types.ts';

const TOKENS_REGEX = /[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)/g;

export function readStr(s: string): MalData {
  const tokens = tokenize(s);
  const reader = new Reader(tokens);
  return readForm(reader);
}

function tokenize(str: string) {
  let re = TOKENS_REGEX;
  let results = [];
  let match;
  while ((match = re && re.exec(str)![1]) !== '') {
    if (match![0] === ';') continue;
    results.push(match);
  }
  const tokens = results.flatMap(s => s ? ({ Token: s }) : []);
  // Validate:
  tokens.forEach(({ Token }) => {
    const firstChar = Token[0];
    const tokenLen = Token.length;
    const lastToken = Token[tokenLen - 1];
    if (firstChar === '"') {
      if (tokenLen === 1 || lastToken !== '"')
        throw new Error('unbalanced')
      if (lastToken === '"') {
        if (Token[tokenLen - 2] === '\\')
          throw new Error('unbalanced')
      }
    }
    if (firstChar === '\\') {
      if (tokenLen === 1)
        throw new Error('unbalanced')
    }
  })
  return tokens;
}

function readForm(reader: Reader): MalData {
  const currentToken = reader.peek();
  const haveCloseDelimiter = closeDelimiter(currentToken)
  if (currentToken && haveCloseDelimiter) {
    const list = readList(haveCloseDelimiter)(reader);
    return list
  }
  return readAtom(reader);
}

type CloseDelimiter = '[' | ']' | '(' | ')';
const readList = (closeDelimiter: CloseDelimiter) => (reader: Reader): MalList => {
  const malList = { MalList: [] as MalData[] };
  while (reader.next()) {
    const currentT = reader.peek();
    if (currentT?.Token === closeDelimiter) {
      return malList
    };
    const malData = readForm(reader);
    malList.MalList.push(malData);
  }
  throw new Error('EOF');
}

function readAtom(reader: Reader): MalAtom {
  const currentToken = reader.peek();
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

function closeDelimiter(t?: Token): CloseDelimiter | undefined {
  switch (t?.Token) {
    case "[":
      return "]"
    case "(":
      return ")"
  }
  return
}
