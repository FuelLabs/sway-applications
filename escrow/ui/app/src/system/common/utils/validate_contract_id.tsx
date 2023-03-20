import { isB256 } from "fuels";

export function validateContractId(address: string) {
  let isError = false;

  if (!isB256(address)) {
    isError = true;
  }

  return { address, isError };
}
