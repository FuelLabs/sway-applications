import { Button } from "@fuel-ui/react";
import { ChangeEvent, useState } from "react";
import { AssetInput } from "./AssetInput";
import { useDeposit } from "../hooks/useDeposit";
import { BigNumberish, bn } from "fuels";

interface Props {
    escrowId: BigNumberish,
}

export function Deposit(props: Props) {
    const [assetAmount, setAssetAmount] = useState("");
    const [assetId, setAssetId] = useState("");
    const depositMutation = useDeposit({
        depositAmount: assetAmount,
        depositAsset: assetId,
        escrowId: bn(props.escrowId)
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