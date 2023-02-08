import { Address } from "fuels";
import { MAX_U64 } from "./constants";
import { bn } from 'fuels'
import { toast } from "@fuel-ui/react";



export function parseAddress(addressInput: string, element: string): string | null {

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
        console.log("failed to parse address input " + element + ": " + addressInput)
        window.alert("Invalid address: " + element)
        return null;
    }

    // If address is a bech32 Fuel address, convert to a hex string
    if (address.slice(0, 4) === "fuel") {
        return parsed.toHexString();
    }

    return address;
}


export function parseAmount(amountInput: string, element: string): string | null {

    // If input element is empty, or cannot be parsed, return null
    if (amountInput === "") {
        return null;
    }

    let parsed;
    try{
        parsed = bn.parseUnits(amountInput, 9);
    }
    catch {
        console.log("failed to parse address input " + element + ": " + amountInput)
        window.alert("Invalid amount: " + element)
        return null;
    }

    //if(parsed.gt(MAX_U64) || parsed.lte(0)){
    //    console.log("size err!")
    //    return null;
    //}

    return parsed.toHex(8);
}
