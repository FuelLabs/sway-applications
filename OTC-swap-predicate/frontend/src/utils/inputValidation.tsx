export function validateAddress(address: string): boolean {
    // TODO Actually check it's a valid address
    return address.length == 66 ? true : false;
}

export function validateAmount(amount: string): boolean {
    // TODO actually check it can be parsed as a number
    return (parseInt(amount) < 2**64 && parseInt(amount) > 0) ? true : false;
}

export function parseAmount(amount: string): string {
    return "0x".concat((parseInt(amount)).toString(16).padStart(16, "0"));
}
