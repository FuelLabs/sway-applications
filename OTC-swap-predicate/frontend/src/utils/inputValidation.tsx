import { Address } from "fuels";
import { MAX_U64 } from "./constants";


export function validateAddress(addressInput: HTMLInputElement | null): string | null {

    // If input element is not initialized, empty, or cannot be parsed as an address, return null
    if (addressInput === null) {
        return null;
    }

    if (addressInput.value=== ""){
        return null;
    }

    let address = addressInput.value;

    let parsed;
    try {
        parsed =  Address.fromAddressOrString(address);
    }
    catch {
        return null;
    }

    // If address is a bech32 Fuel address, convert to a hex string
    if (address.slice(0, 4) == "fuel") {
        return parsed.toHexString();
    }

    return address;

}


export function validateAmount(amountInput: HTMLInputElement | null): string | null {

    // If input element is not initialized, empty, or cannot be parsed, return null
    if(amountInput === null){
        return null;
    }

    if (amountInput.value === "") {
        return null;
    }

    let parsed;
    try{
        parsed = BigInt(amountInput.value);
    }
    catch {
        return null;
    }

    if(parsed > MAX_U64 || parsed < 1){
        return null;
    }

    return "0x".concat(parsed.toString(16).padStart(16, "0"));
}
