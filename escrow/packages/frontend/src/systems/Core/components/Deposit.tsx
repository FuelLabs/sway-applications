import { Flex, Card, Input, Button, toast } from "@fuel-ui/react";
import { ChangeEvent, SyntheticEvent, useState } from "react";
import { walletIndexAtom } from "../jotai";
import { useAtomValue } from "jotai";
import { parseInputValueBigInt } from "../utils/math";
import { useContract } from "../hooks/useContract";
import { queryClient } from "@/queryClient";
import { AssetInput } from "./AssetInput";
import { useDeposit } from "../hooks/useDeposit";

interface Props {
    escrowId: bigint,
}

export function Deposit(props: Props) {
    const [assetAmount, setAssetAmount] = useState("");
    const [assetId, setAssetId] = useState("");
    const contract = useContract();
    const walletIdx = useAtomValue(walletIndexAtom);
    const depositMutation = useDeposit({
        depositAmount: assetAmount,
        depositAsset: assetId,
        escrowId: BigInt(props.escrowId)
    });

    const handleAssetAmountChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newAssetAmount = event.target.value;
        setAssetAmount(newAssetAmount);
    }

    const handleAssetIdChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newAssetId = event.target.value;
        setAssetId(newAssetId);
    }

    return (
        <>
            <AssetInput
                asset={{ assetId, assetAmount }}
                onAssetAmountChange={handleAssetAmountChange}
                onAssetIdChange={handleAssetIdChange}
            />
            <Button
                isDisabled={depositMutation.isLoading}
                isLoading={depositMutation.isLoading}
                onPress={() => depositMutation.mutate()}
                css={{ margin: "10px" }}
            >
                Deposit Asset
            </Button>
        </>
    );
}