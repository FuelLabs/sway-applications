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
  // TODO : GEt provider from wallet extension
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
  const receiverRef = useRef<HTMLInputElement>(null);
  const askTokenRef = useRef<HTMLInputElement>(null);
  const askAmounRef = useRef<HTMLInputElement>(null);

  // Use provided offer conditions to create a corresponding predicate root (address)
  async function handleGenerate() {
    clearResults();

    // Validate inputs    
    let receiverValid = validateAddress(receiverRef.current);
    let askTokenValid = validateAddress(askTokenRef.current);
    let askAmountVald = validateAmount(askAmounRef.current);

    // TO DO : Provide feedback to user on which input(s) were invalid
    if (receiverValid === null || askTokenValid === null|| askAmountVald === null) {
      clearResults();
      return;
    }
    
    // build predicate bytecode, calculate root, and set to state
    let bytecode = buildBytecode(receiverValid, askTokenValid, askAmountVald);
    let predicateAddress = Address.fromString(calculateRoot(bytecode));
    setpredicateAddress(predicateAddress);

    // Look for tokens belonging to the predicate address, and set them to state
    let tokens = await getTokenBalance(predicateAddress, providerAddress);
    setTokensFound(tokens);

  }

  // TODO Spend the tokens found at the predicate address
  async function handleTake() {
  }

  // TODO Recover the tokens found at the predicate address (if owner)
  async function handleCancel() {
  }


  return (
    <>
    <header className="App-header">
      <h1>OTC Swap</h1>
    </header>
    
      <div className="App-main">

        <p>Ask amount</p><input ref={askAmounRef} type="text"/>
        <p>Ask token</p><input ref={askTokenRef} type="text"/>
        <p>Receiver</p><input ref={receiverRef} type="text"/>

        <button className="App-button" onClick={handleGenerate}>Calculate offer address</button>

        <PredicateInfo predicateAddress={predicateAddress} tokensFound={tokensFound} handleTake={handleTake} handleCancel={handleCancel}/>

      </div>
    </>

  );
}
export default App;
