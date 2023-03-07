import { BoxCentered, Button, Flex, Heading, toast, Stack } from "@fuel-ui/react";
import { Address, isBech32, isB256 } from "fuels";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { SignatureComponent } from "../../common/signature";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";
import { IdentityInput } from "../../../contracts/MultisigContractAbi";
import { OptionalCheckBoxComponent } from "../../common/optional_data_checkbox";
import { RadioGroupComponent } from "../../common/radio_group";

export function ExecuteTransactionPage() {
    // Used for our component listeners
    const [address, setAddress] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [data, setData] = useState("")

    const [recipient, setRecipient] = useState("address")
    const [optionalData, setOptionalData] = useState(false)
    const [signatures, setSignatures] = useState([<SignatureComponent id={1} name="transfer" />])
    const { contract, isLoading, isError } = useContract()

    async function useExecuteTransaction() {
        // const signature = document.querySelector<HTMLInputElement>(
        //     `[name="transaction-signature"]`
        // )!.value;

        let identity: IdentityInput;

        if (recipient === "address") {
            let user: string;

            if (isBech32(address)) {
                user = Address.fromString(address).toB256()
            } else if (isB256(address)) {
                user = address;
            } else {
                toast.error("Sir... SIR, that's not a valid address", { duration: 10000 });
                return;
            }

            identity = { Address: { value: user } };
        } else {
            if (!isB256(address)) {
                toast.error("Ha! Take a look at this contract id...", { duration: 10000 });
                return;
            }

            identity = { ContractId: { value: address } };
        }

        let userData = data;
        if (optionalData && !isB256(userData)) {
            toast.error("That data looks a bit off my dude", { duration: 10000 });
            return;
        } else if (!optionalData) {
            userData = "0x0000000000000000000000000000000000000000000000000000000000000000";
        }

        await contract!.functions.execute_transaction(userData, [], identity, assetAmount).call();
        toast.success("Transaction complete!")
    }

    async function addSignature() {
        setSignatures([...signatures, <SignatureComponent id={signatures.length+1} name="transaction" /> ]);
    }

    async function removeSignature() {
        if (signatures.length === 1) {
            toast.error("Cannot remove the last signature")
            return;
        }

        setSignatures([...signatures.splice(0, signatures.length - 1)]);
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "30%" }}>
            <Stack css={{ minWidth: "100%" }}>
                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1" }}>
                    Execute a transaction
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." name="transaction-recipient" />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" name="transaction-value" />

                {signatures.map((signatureComponent, index) => signatureComponent)}

                {optionalData && <InputFieldComponent onChange={setData} text="Optional data" placeholder="0x252afeeb6e..." name="transaction-data" />}
                
                <Button
                    color="accent"
                    onPress={useExecuteTransaction}
                    size="lg"
                    variant="solid"
                    css={{ marginTop: "$1" }}
                >
                    Execute
                </Button>

                <Flex gap="$1" css={{ marginTop: "$1" }}>
                    <Button
                        color="accent"
                        onPress={addSignature}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%" }}
                    >
                        Add signature
                    </Button>

                    <Button
                        color="accent"
                        onPress={removeSignature}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%" }}
                    >
                        Remove signature
                    </Button>
                </Flex>

                <OptionalCheckBoxComponent setOptionalData={setOptionalData} optionalData={optionalData} />
                <RadioGroupComponent setRecipient={setRecipient} />
            </Stack>
        </BoxCentered>
    );
}
