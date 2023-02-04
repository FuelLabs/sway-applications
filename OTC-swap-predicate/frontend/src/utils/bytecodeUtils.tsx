import { calcRoot } from "@fuel-ts/merkle";
import { RAW } from "../precompiles/swapPredicatePrecompile";

export function buildBytecode(receiver: string, askToken: string, askAmount: string): string {
    // Read precompiled binary, and substitute receiver, askToken and askAmount
    // TO DO : This is horrible. Find way to read .bin directly and do substitutions
    let first = RAW.slice(0, 3800);
    let second = RAW.slice(3800+128, 3800+128+48);
    let third = RAW.slice(3800+128+48+16);

    return first.concat(askToken.slice(2)).concat(receiver.slice(2)).concat(second).concat(askAmount.slice(2)).concat(third);
}


export function calculateRoot(bytecode: string): string {
    // TODO: Calculate root of bytecode as split into 8-byte chunks
    let chunks = chunkString(bytecode, 16);
    return calcRoot(chunks);
}

function chunkString (str: string, len: number) {
    const size = Math.ceil(str.length/len);
    const r = Array(size);
    let offset = 0;
    
    for (let i = 0; i < size; i++) {
      r[i] = "0x".concat(str.substr(offset, len));
      offset += len;
    }

    if (r[r.length - 1].length === 10) {
        r[r.length - 1] = r[r.length - 1].concat("00000000");
    }

    return r;
  }
