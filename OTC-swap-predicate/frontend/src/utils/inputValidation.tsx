import { Address } from "fuels";
import { bn } from 'fuels'

export function parseAddress(addressInput: string, element: string): string | null {

    // If input element is empty, or cannot be parsed as an address, return null
    if (addressInput === "") {
        console.log("is empty")
        return null;
    }

    try {
        let parsed = Address.fromDynamicInput(addressInput);
        return parsed.toHexString();
    }

    catch {
        console.log("fails to parse")
        return null;
    }
   
}


export function parseAmount(amountInput: string, element: string): string | null {

    // If input element is empty, or cannot be parsed, return null
    if (amountInput === "") {
        return null;
    }

    try {
        let parsed = bn.parseUnits(amountInput);
        return parsed.toHex(8);
    }
    catch {
        return null;
    }
    
}
