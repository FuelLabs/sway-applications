import {  useState, useRef } from "react";
import { Wallet } from "fuels";
import { buildBytecode, calculateRoot, getTokens } from "./utils/bytecodeUtils";
import {validateAddress, validateAmount, parseAmount} from "./utils/inputValidation";
import "./App.css";

//the private key from createWallet.js
const WALLET_SECRET =
  "0x7b87229cbb3fd7e03e85d95df2be2ab46a2907fcca6ebcc56dc2e34a0abe6cb8";

// Create a Wallet from given secretKey in this case
// The one we configured at the chainConfig.json
const wallet = Wallet.fromPrivateKey(
  WALLET_SECRET,
  "https://node-beta-2.fuel.network/graphql"
);

function App() {
  const [predicateAddress, setpredicateAddress] = useState("");
  const [tokensFound, setTokensFound] = useState([""]);

  const receiverRef = useRef<HTMLInputElement>(null);
  const askTokenRef = useRef<HTMLInputElement>(null);
  const askAmounRef = useRef<HTMLInputElement>(null);

  // Use offer conditions to create a predicate whose bytecode root corresponds to them
  async function handleGenerate() {
    clearResults();
    if (receiverRef.current == null || askTokenRef.current == null || askAmounRef.current == null) return;
    
    const receiver = receiverRef.current.value;
    const askToken = askTokenRef.current.value;
    const askAmount = askAmounRef.current.value;

    // Validate inputs
    if (!validateAddress(receiver) || !validateAddress(askToken)|| !validateAmount(askAmount)) {
      clearResults();
      return;
    }
    
    // build predicate bytecode
    let bytecode = buildBytecode(receiver, askToken, parseAmount(askAmount));

    // Calculate predicate address
    let predicateAddress = calculateRoot(bytecode);
    
    // Set address to state
    setpredicateAddress(predicateAddress);

    // Set the tokens found at the predicate address
    let tokens = getTokens(predicateAddress);
    setTokensFound(tokens);


    function clearResults() {
      setpredicateAddress("");
      setTokensFound([""]);
    }
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

        <button className="App-button" onClick={handleGenerate}>Generate predicate address</button>

        <p>To fund offer, send tokens to :</p>
        <p className="App-address">{predicateAddress}</p>
        <p>Tokens already found at address : {tokensFound.map((token) => token.length > 0 ? <li>{token}</li> : "None")}</p>
      </div>
    </>

  );
}
export default App;