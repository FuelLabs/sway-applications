import { useState, useRef } from "react";
import { Address, CoinQuantity } from "fuels";
import { buildBytecode, calculateRoot } from "./utils/bytecodeUtils";
import { ZERO_ADDRESS } from "./utils/constants";
import { validateAddress, validateAmount } from "./utils/inputValidation";
import { getTokenBalance } from "./utils/predicateBalance";
import PredicateInfo from "./components/predicateInfo";
import "./App.css";


function App() {

  // Set the provider address used for balance queries and transactions
  // TODO : Get provider from wallet extension
  const providerAddress = "https://node-beta-2.fuel.network/graphql"

  // State contains the calculated predicate address and any tokens belonging to it
  const [predicateAddress, setpredicateAddress] = useState(ZERO_ADDRESS);
  const [tokensFound, setTokensFound] = useState<CoinQuantity[]>([]);

  // Convenience function to clear state
  function clearResults() {
    setpredicateAddress(ZERO_ADDRESS);
    setTokensFound([]);
  }

  // References to the input fields
  let receiverRef = useRef<HTMLInputElement>(null);
  let askTokenRef = useRef<HTMLInputElement>(null);
  let askAmountRef = useRef<HTMLInputElement>(null);

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
    setpredicateAddress(predicateAddress);

    // Look for tokens belonging to the predicate address, and set them to state
    let tokens = await getTokenBalance(predicateAddress, providerAddress);
    setTokensFound(tokens);

  }

  // TODO Spend the tokens found at the predicate address
  async function handleTake() {
    window.alert("Not implemented yet!");
  }

  // TODO Recover the tokens found at the predicate address (if owner)
  async function handleCancel() {
    window.alert("Not implemented yet!");
  }


  return (
    <>
    <header className="App-header">
      <h1>OTC Swap</h1>
    </header>
    
      <div className="App-main">
          <p>Ask amount</p><input ref={askAmountRef} id="amountInput" type="text" required/>
          <p>Ask token</p><input ref={askTokenRef} type="text" placeholder="0x... / fuel1..." required/>
          <p>Receiver</p><input ref={receiverRef} type="text" placeholder="0x... / fuel1..." required/>
          <button className="App-button" onClick={handleCalculate}> Calculate offer address </button>

        <PredicateInfo predicateAddress={predicateAddress} tokensFound={tokensFound} handleTake={handleTake} handleCancel={handleCancel}/>

      </div>
    </>

  );
}
export default App;
