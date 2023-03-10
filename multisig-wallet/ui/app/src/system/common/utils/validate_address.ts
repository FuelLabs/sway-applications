import { toast } from "@fuel-ui/react";
import { Address, isBech32, isB256 } from "fuels";

export function validateAddress(address: string) {
    let isError = false;

    if (address.length === 63 && isBech32(address)) {
        address = Address.fromString(address).toB256()
    } else if (isB256(address)) {
        address = address;
    } else {
        toast.error("That address is kinda sus", { duration: 10000 })
        isError = true;
    }

    return { address, isError };
}
