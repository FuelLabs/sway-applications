import { FC, useState } from "react";
import { Address, CoinQuantity } from "fuels";
import { parseAddress, parseAmount } from "../utils/inputValidation";
import getTokenBalance from "../utils/predicateBalance";
import { buildBytecode, calculateRoot } from "../utils/bytecodeUtils";
import { ZERO_ADDRESS } from "../utils/constants";
import { Form, Input, Stack, Tooltip } from "@fuel-ui/react";

export type OfferParams = {
    askAmount: string;
    askToken: string;
    receiver: string;
};

type OfferInputProps = {
    setPredicateAddress: (predicateAddress: Address) => void;
    setTokensFound: (tokens: CoinQuantity[]) => void;
    network: string;
};

const OfferInput: FC<OfferInputProps> = ({
    setPredicateAddress,
    setTokensFound,
    network,
}: OfferInputProps) => {

    // State to track whether inputs have been initialized and are valid
    // Only render address and token list once inputs are valid
    // Only render input as error when invalid but initialized
    const [askTokenInitialized, setAskTokenInitialized] = useState(false);
    const [askTokenValid, setAskTokenValid] = useState(true);
    const [receiverInitialized, setReceiverInitialized] = useState(false);
    const [receiverValid, setReceiverValid] = useState(true);

    // Use provided offer conditions to create a corresponding predicate root (address)
    async function handleCalculate() {

        const askAmount = document.querySelector<HTMLInputElement>(
            `.input [name="amount"]`
        )!.value;

        const askToken = document.querySelector<HTMLInputElement>(
            `.input [name="token"]`
        )!.value;

        const receiver = document.querySelector<HTMLInputElement>(
            `.input [name="receiver"]`
        )!.value;

        
        if (!askTokenInitialized && askToken !== "") {
            setAskTokenInitialized(true);
        }
        
        if (!receiverInitialized && receiver !== "") {
            setReceiverInitialized(true);
        }

        // Validate inputs
        let askAmountParsed = parseAmount(askAmount, "ask amount");
        let askTokenParsed = parseAddress(askToken, "ask token");
        let receiverParsed = parseAddress(receiver, "receiver");

        setAskTokenValid(askTokenParsed !== null);
        setReceiverValid(receiverParsed !== null);

        // If any not valid, set predicateAddress to ZERO_ADDRESS
        if (!askTokenValid || !receiverValid || askAmountParsed === null) {
            setPredicateAddress(ZERO_ADDRESS);
            return;
        }

        // build predicate bytecode, calculate root, and set to state
        let bytecode = buildBytecode(
            receiverParsed!,
            askTokenParsed!,
            askAmountParsed!
        );
        let predicateAddress = Address.fromString(calculateRoot(bytecode));
        setPredicateAddress(predicateAddress);

        // Look for tokens belonging to the predicate address, and set them to state
        let tokens = await getTokenBalance(predicateAddress, network);
        setTokensFound(tokens);
    }

    return (
        <Stack css={{ minWidth: "30%", maxWidth: "80%" }}>
            <Form.Control className="input" isRequired>
                <Form.Label css={{ color: "white" }} htmlFor="amount">
                    Requested token amount
                </Form.Label>
                <Tooltip
                    content={
                        <>The amount of tokens you would like to receive</>
                    }
                >
                    <Input key="amount" size="lg" isFullWidth>
                        <Input.Number
                            inputMode="decimal"
                            name="amount"
                            placeholder="0.0"
                            onChange={handleCalculate}
                        />
                    </Input>
                </Tooltip>
                <Form.Label css={{ color: "white" }} htmlFor="token">
                    Requested Asset ID
                </Form.Label>
                <Tooltip
                    content={
                        <>The asset ID of tokens you would like to receive</>
                    }
                >
                    <Input
                        key="token"
                        size="lg"
                        isFullWidth
                        // p: askTokenInitialized
                        // q: askTokenValid
                        // isValid = p && q
                        // => isInvalid = !p || !q
                        // so this can be changed as isInvalid = !askTokenInitialized || !askTokenValid 
                        isInvalid={askTokenInitialized ? !askTokenValid: false} 
                    >
                        <Input.Field
                            inputMode="text"
                            name="token"
                            placeholder="0x... / fuel1..."
                            onChange={handleCalculate}
                        />
                    </Input>
                </Tooltip>
                <Form.Label css={{ color: "white" }} htmlFor="receiver">
                    Receiver
                </Form.Label>
                <Tooltip
                    content={
                        <>
                            The address you would like to receive the funds at.
                            Note: This is also the only address that will be
                            able to cancel the order
                        </>
                    }
                >
                    <Input
                        key="receiver"
                        size="lg"
                        isFullWidth
                        isInvalid={receiverInitialized ? !receiverValid: false}
                    >
                        <Input.Field
                            inputMode="text"
                            name="receiver"
                            placeholder="0x... / fuel1..."
                            onChange={handleCalculate}
                        />
                    </Input>
                </Tooltip>
            </Form.Control>
        </Stack>
    );
};

export default OfferInput;
