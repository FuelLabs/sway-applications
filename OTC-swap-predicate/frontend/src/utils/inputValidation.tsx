import { Address } from "fuels";


export function validateAddress(address: string): boolean {
    return Address.fromAddressOrString(address) !== null;
}


// TODO : Amounts should be BigNumbers. Validation and parsing need to account for this
export function validateAmount(amount: string): boolean {
    // TODO actually check it can be parsed as a number
    let parsed = parseInt(amount);
    return (parsed < 2**64 && parsed > 0) ? true : false;
}

export function parseAmount(amount: string): string {
    return "0x".concat((parseInt(amount)).toString(16).padStart(16, "0"));
}
