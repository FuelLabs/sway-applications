import { toast } from "@fuel-ui/react";
import { Address, isBech32, isB256 } from "fuels";

export function validateAddress(address: string) {
    let isError = false;

    if (isBech32(address)) {
        address = Address.fromString(address).toB256()
    } else if (isB256(address)) {
        address = address;
    } else {
        // toast.error("Sir... SIR, that's not a valid address", { duration: 10000 });
        isError = true;
    }

    return { address, isError };
}
