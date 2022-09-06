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

    const handleDeposit = (event: any) => {
        const actualDeposit = parseInputValueBigInt(assetAmount);
        const result = contract!
            .multiCall([
                contract!.functions.deposit(props.escrowId).callParams({
                    forward: [actualDeposit, assetId]
                }),
            ])
            .txParams({
                gasPrice: BigInt(5),
                bytePrice: BigInt(5),
                gasLimit: 100_000_000
            }).call();
        toast.promise(result, {
            loading: 'Transaction loading...',
            success: 'Desposited successfully',
            error: 'Transaction reverted!'
        });
        setAssetAmount("");
        setAssetId("");
        queryClient.fetchQuery(['EscrowPage-balances', walletIdx]);
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