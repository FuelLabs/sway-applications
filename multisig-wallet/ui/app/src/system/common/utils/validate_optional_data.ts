import { toast } from "@fuel-ui/react";
import { isB256 } from "fuels";

export function validateOptionalData(data: string) {
    let isError = false;

    let validatedData: string | undefined = data;
    if (data === "") {
        validatedData = undefined;
    } else if (!isB256(data)) {
        toast.error("That data looks a bit off my dude", { duration: 10000 });
        isError = true;
    }

    return { validatedData, isError };
}
