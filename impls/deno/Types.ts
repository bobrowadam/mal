export type MalData = MalList | MalAtom;
export type MalList = { MalList: MalData[] };

export type MalAtom = { MalNumber: number } | { MalString: string } | { MalSymbol: symbol } | { MalNil: 'Nil' };
export type Token = { Token: string };
