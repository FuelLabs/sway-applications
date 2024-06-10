import crypto from "crypto";
import { AssetIdInput } from "@/contract-types/contracts/NFTContractAbi";

// creates a subId repeating the provided number
export function createSubId(numberToRepeat: number) {
    return numberToRepeat.toString().repeat(32);
}

export function createAssetId(subId: string, contractId: string) {
    const hasher = crypto.createHash("sha256");
    hasher.update(contractId);
    hasher.update(subId);
    const assetId: AssetIdInput = {
        bits: `0x${hasher.digest('hex')}`
    }
    return assetId
}
