import { Address } from "fuels";


export function validateAddress(addressInput: HTMLInputElement | null): string | null {

    // If input element is not initialized, empty, or cannot be parsed, return null
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

    // If address is a fuel address, convert to a hex string
    if (address.slice(0, 4) == "fuel") {
        return parsed.toHexString();
    }

    return address;

}


// TODO : Amounts should be BigNumbers. Validation and parsing need to account for this
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
        parsed = parseInt(amountInput.value);
    }
    catch {
        return null;
    }

    if(parsed > 2**64 - 1 || parsed < 1){
        return null;
    }

    return "0x".concat(parsed.toString(16).padStart(16, "0"));
}
