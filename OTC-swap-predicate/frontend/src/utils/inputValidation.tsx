import { Address } from "fuels";
import { MAX_U64 } from "./constants";


export function parseAddress(addressInput: string): string | null {

    // If input element is empty, or cannot be parsed as an address, return null
    if (addressInput === ""){
        return null;
    }

    let address = addressInput;

    let parsed;
    try {
        parsed =  Address.fromAddressOrString(address);
    }
    catch {
        console.log( "failed to validate input: " + addressInput)
        return null;
    }

    // If address is a bech32 Fuel address, convert to a hex string
    if (address.slice(0, 4) === "fuel") {
        return parsed.toHexString();
    }

    return address;
}


export function parseAmount(amountInput: string): string | null {

    // If input element is empty, or cannot be parsed, return null
    if (amountInput === "") {
        return null;
    }

    let parsed;
    try{
        parsed = BigInt(amountInput);
    }
    catch {
        console.log( "failed to validate input: " + amountInput)
        return null;
    }

    if(parsed > MAX_U64 || parsed < 1){
        return null;
    }

    return "0x".concat(parsed.toString(16).padStart(16, "0"));
}
