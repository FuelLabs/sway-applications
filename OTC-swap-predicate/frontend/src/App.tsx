import {  useState, useRef } from "react";
import { Address} from "fuels";
import { buildBytecode, calculateRoot } from "./utils/bytecodeUtils";
import { getTokenBalance } from "./utils/predicateBalance";
import {validateAddress, validateAmount} from "./utils/inputValidation";
import "./App.css";


function App() {

  // Set the provider address used for balance queries and transactions
  const providerAddress = "https://node-beta-2.fuel.network/graphql"

  // State contains the calculated predicate address and any tokens belonging to it
  const [predicateAddress, setpredicateAddress] = useState("");
  const [tokensFound, setTokensFound] = useState([{"asset_id": "", "amount": ""}]);

  // Convenience function to clear state
  function clearResults() {
    setpredicateAddress("");
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

    // TO DO : Make it clear which input was invalid
    if (receiverValid === null || askTokenValid === null|| askAmountVald === null) {
      clearResults();
      return;
    }
    
    // build predicate bytecode, calculate root, and set to state
    let bytecode = buildBytecode(receiverValid, askTokenValid, askAmountVald);
    let predicateAddress = calculateRoot(bytecode);
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

        <button className="App-button" onClick={handleGenerate}>Generate offer address</button>

        {/* Only render this part if the predicateAddress has been calculated */}
        {predicateAddress.length > 0 &&
          <>
            <p>To fund this offer, send tokens to :</p>
            <p className="App-address">{Address.fromAddressOrString(predicateAddress).toString()}</p>

            {/* Only render this part if tokens are found */}
            {tokensFound.length > 0 &&
              <>
                <p>Tokens found at address :</p>
                <table className="App-tokenTable">
                  <thead>
                    <tr key="headers">
                      <th>Asset ID</th>
                      <th>Amount</th>
                    </tr>
                  </thead>
                  <tbody>
                    {tokensFound.map((token) => (
                      <tr key="items">
                        <td className="App-address">{token.asset_id}</td>
                        <td>{token.amount}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>

                <div>
                  <button className="App-button" onClick={handleTake}>Take offer</button>
                  <button className="App-button" onClick={handleCancel}>Cancel offer</button>
                </div>

              </>
            } 
          </>
        }
    
      </div>
    </>

  );
}
export default App;
