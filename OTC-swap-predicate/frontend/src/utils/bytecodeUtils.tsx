import { calcRoot } from "@fuel-ts/merkle";
import { RAW, OFFSETS } from "../precompiles/swapPredicatePrecompile";

// Read precompiled binary, and substitute receiver, askToken and askAmount
export function buildBytecode(receiver: string, askToken: string, askAmount: string): string {
    
    // Note: If the order of the values changes in the bytecode, this will break
    // Addresses are 32 bytes (64 chars), U64 amounts are 8 bytes (16 chars)
    const BEFORE_ASK_TOKEN = RAW.slice(0, OFFSETS.ASK_TOKEN);
    const ASK_TO_RECEIVER = RAW.slice(OFFSETS.ASK_TOKEN + 64, OFFSETS.RECEIVER);
    const RECEIVER_TO_ASK_AMOUNT = RAW.slice(OFFSETS.RECEIVER + 64, OFFSETS.ASK_AMOUNT);
    const AFTER_ASK_AMOUNT = RAW.slice(OFFSETS.ASK_AMOUNT + 16);

    return BEFORE_ASK_TOKEN
    .concat(askToken.slice(2))
    .concat(ASK_TO_RECEIVER)
    .concat(receiver.slice(2))
    .concat(RECEIVER_TO_ASK_AMOUNT)
    .concat(askAmount.slice(2))
    .concat(AFTER_ASK_AMOUNT);

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
