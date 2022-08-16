import { Flex, Card, Input, Button, toast } from "@fuel-ui/react";
import { ChangeEvent, SyntheticEvent, useState } from "react";
import { walletIndexAtom } from "../jotai";
import { useAtomValue } from "jotai";
import { parseInputValueBigInt } from "../utils/math";
import { useContract } from "../hooks/useContract";
import { queryClient } from "@/queryClient";
import { AssetInput } from "./AssetInput";

export function Deposit() {
    const [assetAmount, setAssetAmount] = useState("");
    const [assetId, setAssetId] = useState("");
    const contract = useContract();
    const walletIdx = useAtomValue(walletIndexAtom);

    const handleAssetAmountChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newAssetAmount = event.target.value;
        setAssetAmount(newAssetAmount);
    }

    const handleAssetIdChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newAssetId = event.target.value;
        setAssetId(newAssetId);
    }

    const handleDeposit = (event: any) => {
        event.preventDefault();
        const actualDeposit = parseInputValueBigInt(assetAmount);
        const result = contract!.functions.deposit(actualDeposit).call()
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
        <Flex css={{ flex: "1", justifyContent: "center" }}>
            <Card css={{ margin: "10px", bg: "$gray7", marginTop: "50px", width: "450px" }}>
                <AssetInput
                    asset={{ assetId, assetAmount }}
                    onAssetAmountChange={handleAssetAmountChange}
                    onAssetIdChange={handleAssetIdChange}
                />
                <Button onPress={(e) => handleDeposit(e)} css={{ margin: "10px" }}>Deposit Asset</Button>
            </Card>
        </Flex>
    );
}