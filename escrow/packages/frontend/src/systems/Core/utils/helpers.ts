import { formatUnits } from "ethers/lib/utils";

export   const formatValue = (amount: bigint | null | undefined, decimals: number) => {
  if (amount != null) {
    return formatUnits(amount, decimals);
  }
  return "";
};