import { AssetIdInput } from "@/contract-types/contracts/NFTContractAbi";
import { arrayify, concat, sha256 } from "fuels";

// creates a subId repeating the provided number
export function createSubId(numberToRepeat: number) {
    return numberToRepeat.toString().repeat(32);
}

export function createAssetId(subId: string, contractId: string) {
    const contractIdBytes = arrayify(contractId);
    const subIdBytes = arrayify(subId);
    const bits = sha256(concat([contractIdBytes, subIdBytes]));
    const assetId: AssetIdInput = {
        bits
    }
    return assetId
}
