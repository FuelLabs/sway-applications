import { toast } from "@fuel-ui/react";
import { isB256 } from "fuels";

export function validateContractId(address: string) {
  let isError = false;

  if (!isB256(address)) {
    toast.error("Ha! Take a look at this contract id...", { duration: 10000 });
    isError = true;
  }

  return { address, isError };
}
