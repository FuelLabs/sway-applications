import { BoxCentered, Button, Flex, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { ContractIdInput } from "../../../contracts/MultisigContractAbi";
import { InputFieldComponent } from "../../common/components/input_field";
import { ButtonComponent } from "../../common/components/button";
import { validateAddress } from "../../common/utils/validate_address";
import { validateContractId } from "../../common/utils/validate_contract_id";
import { useIsConnected } from "../../core/hooks/useIsConnected";

export function ViewPage() {
    const [address, setAddress] = useState("")
    const [asset, setAsset] = useState("")

    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function getBalance() {
        let { address: userAsset, isError } = validateContractId(asset);
        if (isError) return;

        let assetId: ContractIdInput = {
            value: userAsset
        }

        try {
            const { value } = await contract!.functions.balance(assetId).get();
            toast.success(`Balance: ${value}`, { duration: 10000 });
        } catch (err) {
            toast.error("The contract did an oopsie", { duration: 10000 });
        }
    }

    async function getNonce() {
        try {
            const { value } = await contract!.functions.nonce().get();
            toast.success(`Current nonce: ${value}`, { duration: 10000 });
        } catch (err) {
            toast.error("Sorry, contract is playing hard to get", { duration: 10000 });
        }
    }

    async function getThreshold() {
        try {
            const { value } = await contract!.functions.threshold().get();
            toast.success(`Threshold: ${value}`, { duration: 10000 });
        } catch (err) {
            toast.error("The matrix is glitching out, can't get the value", { duration: 10000 });
        }
    }

    async function getWeight() {
        let { address: user, isError } = validateAddress(address);
        if (isError) return;

        try {
            const { value } = await contract!.functions.approval_weight(user).get();
            toast.success(`User weight: ${value}`, { duration: 10000 });
        } catch (err) {
            toast.error("I'm not feeling great rn, try again later?", { duration: 10000 });
        }
    }

    async function cancelTransaction() {
        try {
            await contract!.functions.cancel_transaction().call();
            toast.success("Cancelled transaction.", { duration: 10000 });
        } catch (err) {
            toast.error("I don't know how to tell you this but there was a problem...", { duration: 10000 });
        }
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>
            <Stack css={{ minWidth: "100%" }}>
                <Stack>
                    <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                        Check user approval weight
                    </Heading>

                    <InputFieldComponent onChange={setAddress} text="User address" placeholder="0x80d5e8c2be..." />
                    <ButtonComponent handler={getWeight} isConnected={isConnected} text="Get weight" />
                </Stack>

                <Stack css={{ minWidth: "100%", marginTop: "$10" }}>
                    <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                        Check balance of asset
                    </Heading>

                    <InputFieldComponent onChange={setAsset} text="Asset id" placeholder="0x0000000000..." />
                    <ButtonComponent handler={getBalance} isConnected={isConnected} text="Get balance" />
                </Stack>

                <Flex gap="$2" css={{ minWidth: "100%", marginTop: "$10" }}>
                    <Stack css={{ minWidth: "50%" }}>
                        <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1" }}>
                            Check nonce
                        </Heading>

                        <ButtonComponent handler={getNonce} isConnected={isConnected} text="Get nonce" />
                    </Stack>

                    <Stack css={{ minWidth: "50%" }}>
                        <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$14", color: "$accent1" }}>
                            Check threshold
                        </Heading>

                        <ButtonComponent handler={getThreshold} isConnected={isConnected} text="Get threshold" />
                    </Stack>
                </Flex>

                <Stack css={{ minWidth: "100%", marginTop: "$24" }}>
                    <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                        Cancel next transaction
                    </Heading>

                    <ButtonComponent handler={cancelTransaction} isConnected={isConnected} text="Cancel transaction" />
                </Stack>
            </Stack>
        </BoxCentered>
    );
}
