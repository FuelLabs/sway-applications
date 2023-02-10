import { Address } from "fuels";
import { bn } from 'fuels'



export function parseAddress(addressInput: string, element: string): string | null {

    // If input element is empty, or cannot be parsed as an address, return null
    if (addressInput === "") {
        return null;
    }

    let parsed;

    if (addressInput.slice(0, 5) === "fuel1") {
        try {
            parsed = Address.fromAddressOrString(addressInput);
        }
        catch {
            return null;
        }
    }

    else {
        try {
            parsed = Address.fromDynamicInput(addressInput);
        }

        catch {
            return null;
        }
    }

    return parsed.toHexString();
}


export function parseAmount(amountInput: string, element: string): string | null {

    // If input element is empty, or cannot be parsed, return null
    if (amountInput === "") {
        return null;
    }

    let parsed;
    try {
        parsed = bn.parseUnits(amountInput);
    }
    catch {
        return null;
    }

    return parsed.toHex(8);
}
