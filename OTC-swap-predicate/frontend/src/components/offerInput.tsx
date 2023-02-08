import { FC, useRef } from "react";
import { Address, CoinQuantity } from "fuels";
import { validateAddress, validateAmount } from "../utils/inputValidation";
import getTokenBalance from "../utils/predicateBalance";
import { buildBytecode, calculateRoot } from "../utils/bytecodeUtils";
import { ZERO_ADDRESS } from "../utils/constants";


type OfferInputProps = {
    setPredicateAddress: (predicateAddress: Address) => void,
    setTokensFound: (tokens: CoinQuantity[]) => void,
    network: string,
}

const OfferInput: FC<OfferInputProps> = ({setPredicateAddress, setTokensFound, network}: OfferInputProps) => {

    // References to the input fields
    let receiverRef = useRef<HTMLInputElement>(null);
    let askTokenRef = useRef<HTMLInputElement>(null);
    let askAmountRef = useRef<HTMLInputElement>(null);

    
    // Convenience function to clear state
    function clearResults() {
        setPredicateAddress(ZERO_ADDRESS);
        setTokensFound([]);
    }

    // Use provided offer conditions to create a corresponding predicate root (address)
    async function handleCalculate() {
        clearResults();

        // Validate inputs    
        let receiverValid = validateAddress(receiverRef.current);
        let askTokenValid = validateAddress(askTokenRef.current);
        let askAmountValid = validateAmount(askAmountRef.current);


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
        <>
        <p>Ask amount</p><input ref={askAmountRef} id="amountInput" type="text" required/>
        <p>Ask token</p><input ref={askTokenRef} type="text" placeholder="0x... / fuel1..." required/>
        <p>Receiver</p><input ref={receiverRef} type="text" placeholder="0x... / fuel1..." required/>
        <button className="App-button" onClick={handleCalculate}> Calculate offer address </button>
        </>
    );
}


export default OfferInput;
