//import { ESCROW_PATH } from "@/config";
import { DECIMAL_PRECISION } from "@/config";
import { ArbiterInput, AssetInput, IdentityInput } from "@/types/contracts/EscrowAbi";
import { Button, Stack, Input, Card, Flex } from "@fuel-ui/react";
import { ChangeEvent, SyntheticEvent, useState } from "react";
import { useQueryClient } from "react-query";
import { useAtomValue } from "jotai";
import { useWallet } from "../context/AppContext";
import { useContract } from "../hooks/useContract";

import { walletIndexAtom } from "../jotai";
import { ArbiterInputContainer } from "./ArbiterInputContainer"
import { AssetInputContainer } from "./AssetInputContainer";

export const CreateEscrow = () => {
    const queryClient = useQueryClient();
    const walletIdx = useAtomValue(walletIndexAtom);
    const wallet = useWallet();
    const contract = useContract();
    const [arbiter, setArbiter] = useState("");
    const [arbiterAsset, setArbiterAsset] = useState("");
    const [arbiterFee, setArbiterFee] = useState<number>();
    const [buyer, setBuyer] = useState("");
    const [deadline, setDeadline] = useState<number>();
    const [assets, setAssets] = useState([{
        assetId: "",
        assetAmount: ""
    }]);

    const handleArbiterAddressChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newArbiter = event.target.value;
        setArbiter(newArbiter);
    }

    const handleArbiterAssetChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newAssetId = event.target.value;
        setArbiterAsset(newAssetId);
    }

    const handleArbiterFeeChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newFee = event.target.value;
        setArbiterFee(parseInt(newFee));
    }

    const handleBuyerAddressChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newBuyer = event.target.value;
        setBuyer(newBuyer);
    }

    const handleDeadlineChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newDeadline = event.target.value;
        setDeadline(parseInt(newDeadline));
    }

    const handleAssetIdChange = (event: ChangeEvent<HTMLInputElement>, assetIdx: number) => {
        const newAssets = [...assets];
        newAssets[assetIdx].assetId = event.target.value;
        setAssets(newAssets);
    }

    const handleAssetAmountChange = (event: ChangeEvent<HTMLInputElement>, assetIdx: number) => {
        const newAssets = [...assets];
        newAssets[assetIdx].assetAmount = event.target.value;
        setAssets(newAssets);
    }

    const handleAddAsset = () => {
        setAssets([...assets, { assetId: "", assetAmount: "" }]);
    }

    const handleRemoveAsset = (assetIdx: number) => {
        setAssets(assets.filter((asset, i) => {
            return i !== assetIdx;
        }));
    }

    const handleSubmit = async (event: SyntheticEvent) => {
        event.preventDefault();
        // TODO make this more flexible for assets of arbitrary decimal precision
        const actualFee = BigInt(arbiterFee!) * BigInt(DECIMAL_PRECISION);
        let arbiterArg: ArbiterInput = {
            address: { Address: { value: arbiter} },
            asset: { value: arbiterAsset },
            fee_amount: actualFee,
        };
        // TODO make this more flexible when escrow takes an arbitrary amount of assets as input
        // TODO multiply asset amount by DECIMAL_PRECISION
        let assetsArg: [AssetInput, AssetInput] = [
            { amount: assets[0].assetAmount, id: { value: assets[0].assetId } },
            { amount: assets[1].assetAmount, id: { value: assets[1].assetId } }
        ];
        // TODO how to pass buyer as either an Address OR a ContractId?
        let buyerArg: IdentityInput = {
            Address: { value: buyer }
        };
        // TODO change this from multiCall to single call once https://github.com/FuelLabs/fuels-ts/issues/445
        // is fixed
        // TODO don't hardcode gas and byte prices
        const result = await contract!
            .multiCall([
                contract!.functions.create_escrow(arbiterArg, assetsArg, buyerArg, deadline!).callParams({
                    forward: [actualFee, arbiterAsset]
                }),
            ])
            .txParams({
                gasPrice: BigInt(5),
                bytePrice: BigInt(5),
                gasLimit: 100_000_000
            }).call();
        console.log(result);
        setArbiter("");
        setArbiterAsset("");
        // TODO use react number format
        setArbiterFee(undefined);
        setBuyer("");
        // TODO user react number format
        setDeadline(undefined);
        setAssets([{ assetAmount: "", assetId: ""}]);
        // Trigger query to update show balances component
        queryClient.fetchQuery(['EscrowPage-balances', walletIdx]);
    }

    return (
        <Flex css={{ flex: "1", justifyContent: "center" }}>
            <Card css={{ margin: "10px", bg: "$gray7", marginTop: "50px" }}>
                <form onSubmit={handleSubmit}>
                    <Stack css={{ width: "475px", margin: "10px", alignItems: "center" }}>
                        <ArbiterInputContainer
                            onArbiterAddressChange={handleArbiterAddressChange}
                            onAssetIdChange={handleArbiterAssetChange}
                            onFeeChange={handleArbiterFeeChange}
                            arbiterAddress={arbiter}
                            asset={arbiterAsset}
                            feeAmount={arbiterFee}
                        />
                        <Input css={{ alignSelf: "stretch" }} >
                            <Input.Field
                                id={`buyerAddress`}
                                name={`buyerAddress`}
                                placeholder={`Buyer Address`}
                                value={buyer}
                                type="text"
                                onChange={(e) => handleBuyerAddressChange(e)}
                                css={{ font: "$sans" }}
                            />
                        </Input>
                        <Input css={{ alignSelf: "stretch" }} >
                            <Input.Field
                                id={`deadline`}
                                name={`deadline`}
                                placeholder={`Escrow Deadline (block number)`}
                                value={deadline}
                                type="number"
                                onChange={(e) => handleDeadlineChange(e)}
                                css={{ font: "$sans" }}
                            />
                        </Input>
                        <AssetInputContainer
                            onAddAsset={handleAddAsset}
                            onRemoveAsset={handleRemoveAsset}
                            onAssetAmountChange={handleAssetAmountChange}
                            onAssetIdChange={handleAssetIdChange}
                            assets={assets}
                        />
                        <Button type="submit" leftIcon="PlusIcon" css={{ font: "$sans", alignSelf: "stretch" }}>Create Escrow</Button>
                    </Stack>
                </form>
            </Card>
        </Flex>
    );
}