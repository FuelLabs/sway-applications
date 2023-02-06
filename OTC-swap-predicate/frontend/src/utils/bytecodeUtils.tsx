import { calcRoot } from "@fuel-ts/merkle";
import { PRECOMPILE } from "../precompiles/swapPredicatePrecompile";

// Read precompiled binary, and substitute receiver, askToken and askAmount
export function buildBytecode(receiver: string, askToken: string, askAmount: string): string {
    
    // Addresses are 32 bytes (64 chars), U64 amounts are 8 bytes (16 chars)
    let bytecode = PRECOMPILE.BYTECODE;
    bytecode = bytecode.slice(0, PRECOMPILE.ASK_TOKEN_OFFSET).concat(askToken.slice(2)).concat(bytecode.slice(PRECOMPILE.ASK_TOKEN_OFFSET + 64));
    bytecode = bytecode.slice(0, PRECOMPILE.RECEIVER_OFFSET).concat(receiver.slice(2)).concat(bytecode.slice(PRECOMPILE.RECEIVER_OFFSET + 64));
    bytecode = bytecode.slice(0, PRECOMPILE.ASK_AMOUNT_OFFSET).concat(askAmount.slice(2)).concat(bytecode.slice(PRECOMPILE.ASK_AMOUNT_OFFSET + 16));

    return bytecode;
}


// Calculate the Merkle root of the bytecode. Each leaf is 8 bytes.
export function calculateRoot(bytecode: string): string {
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

    // If final leaf is not 8 bytes, pad with 0s
    r[r.length - 1] = r[r.length - 1].padEnd(18, "0");

    return r;
  }
