import { FC } from "react";
import { Address, CoinQuantity } from "fuels";
import { parseAddress, parseAmount } from "../utils/inputValidation";
import getTokenBalance from "../utils/predicateBalance";
import { buildBytecode, calculateRoot } from "../utils/bytecodeUtils";
import { ZERO_ADDRESS } from "../utils/constants";
import { Form, Input, Stack, Button } from "@fuel-ui/react";


type OfferInputProps = {
    setPredicateAddress: (predicateAddress: Address) => void,
    setTokensFound: (tokens: CoinQuantity[]) => void,
    network: string,
}

const OfferInput: FC<OfferInputProps> = ({setPredicateAddress, setTokensFound, network}: OfferInputProps) => {

    // Convenience function to get input field values
    function getParams() : {askAmount: string, askToken: string, receiver: string} {
        const askAmount = document.querySelector<HTMLInputElement>(`.input [name="amount"]`)!.value;
        const askToken = document.querySelector<HTMLInputElement>(`.input [name="token"]`)!.value;
        const receiver = document.querySelector<HTMLInputElement>(`.input [name="receiver"]`)!.value;
        return {askAmount, askToken, receiver}
    }
    
    // Convenience function to clear state
    function clearResults() {
        setPredicateAddress(ZERO_ADDRESS);
        setTokensFound([]);
    }

    // Use provided offer conditions to create a corresponding predicate root (address)
    async function handleCalculate() {
        clearResults();

        let {askAmount, askToken, receiver} = getParams();

        // Validate inputs    
        let askAmountValid = parseAmount(askAmount);
        let askTokenValid = parseAddress(askToken);
        let receiverValid = parseAddress(receiver);

        // TODO : Provide feedback to user on which input(s) were invalid
        if (receiverValid === null || askTokenValid === null|| askAmountValid === null) {
        clearResults();
        return;
        }
        
        // build predicate bytecode, calculate root, and set to state
        let bytecode = buildBytecode(receiverValid, askTokenValid, askAmountValid);
        let predicateAddress = Address.fromString(calculateRoot(bytecode));
        setPredicateAddress(predicateAddress);

        // Look for tokens belonging to the predicate address, and set them to state
        let tokens = await getTokenBalance(predicateAddress, network);
        setTokensFound(tokens);
    }

    return (
        <Stack css={{ maxW: "400px" }}>
            <Form.Control className="input" isRequired>
                <Form.Label css={{color: "white"}} htmlFor="amount">
                    Ask amount
                </Form.Label>
                <Input key="amount" size="lg" isFullWidth >
                    <Input.Number inputMode="decimal" name="amount" placeholder="0.0"/>
                </Input>
                <Form.Label css={{color: "white"}} htmlFor="token">
                    Ask token
                </Form.Label>
                <Input key="token" size="lg" isFullWidth>
                    <Input.Field inputMode="text" name="token" placeholder="0x... / fuel1..."/>
                </Input>
                <Form.Label css={{color: "white"}} htmlFor="receiver">
                    Ask receiver
                </Form.Label>
                <Input key="receiver" size="lg" isFullWidth>
                    <Input.Field inputMode="text" name="receiver" placeholder="0x... / fuel1..."/>
                </Input>
            </Form.Control>
            <Button onPress={handleCalculate}> Calculate offer address </Button>
        </Stack>
    );
}


export default OfferInput;
