import { calcRoot } from "@fuel-ts/merkle";
import { RAW, OFFSETS } from "../precompiles/swapPredicatePrecompile";

// Read precompiled binary, and substitute receiver, askToken and askAmount
export function buildBytecode(receiver: string, askToken: string, askAmount: string): string {
    
    // Addresses are 32 bytes (64 chars), U64 amounts are 8 bytes (16 chars)
    let bytecode = RAW;
    bytecode = bytecode.slice(0, OFFSETS.ASK_TOKEN).concat(askToken.slice(2)).concat(bytecode.slice(OFFSETS.ASK_TOKEN + 64));
    bytecode = bytecode.slice(0, OFFSETS.RECEIVER).concat(receiver.slice(2)).concat(bytecode.slice(OFFSETS.RECEIVER + 64));
    bytecode = bytecode.slice(0, OFFSETS.ASK_AMOUNT).concat(askAmount.slice(2)).concat(bytecode.slice(OFFSETS.ASK_AMOUNT + 16));

    return bytecode;


}


export function calculateRoot(bytecode: string): string {
    // TODO: Calculate root of bytecode as split into 8-byte chunks
    let chunks = chunkString(bytecode, 16);
    return calcRoot(chunks);
}

// Break a string into an array of substrings of a given length
function chunkString (str: string, len: number) {
    const size = Math.ceil(str.length/len);
    const r = Array(size);
    let offset = 0;
    
    for (let i = 0; i < size; i++) {
      r[i] = "0x".concat(str.substring(offset, offset + len));
      offset += len;
    }

    if (r[r.length - 1].length === 10) {
        r[r.length - 1] = r[r.length - 1].concat("00000000");
    }

    return r;
  }
