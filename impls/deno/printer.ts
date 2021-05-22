import { MalData } from './Types.ts';

export function printStr(mal: MalData): string {
  if ('MalList' in mal) {
    return '(' + mal.MalList.map(printStr).join(' ') + ')'
  }
  if ('MalNumber' in mal) {
    return mal.MalNumber.toString();
  }
  if ('MalString' in mal) {
    return mal.MalString;
  }
  if ('MalSymbol' in mal) {
    return mal.MalSymbol.toString();
  }
  if ('MalNil' in mal) {
    return ''
  }
  throw new Error('Unexpected case');
}
