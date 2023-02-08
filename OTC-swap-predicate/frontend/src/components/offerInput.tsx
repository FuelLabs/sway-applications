import { FC } from "react";


type OfferInputProps = {
    askAmountRef: React.RefObject<HTMLInputElement>,
    askTokenRef: React.RefObject<HTMLInputElement>,
    receiverRef: React.RefObject<HTMLInputElement>,
    handleCalculate: () => void,
}

const OfferInput: FC<OfferInputProps> = ({askAmountRef, askTokenRef, receiverRef, handleCalculate}: OfferInputProps) => {
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
