import { useState } from "react";
import { CoinQuantity } from "fuels";
import { ZERO_ADDRESS } from "./utils/constants";
import OfferInput from "./components/offerInput";
import PredicateInfo from "./components/predicateInfo";
import useNetwork from "./hooks/useNetwork";

import "./App.css";
import { Heading } from "@fuel-ui/react";



function App() {

  // State contains the calculated predicate address and any tokens belonging to it
  const [predicateAddress, setPredicateAddress] = useState(ZERO_ADDRESS);
  const [tokensFound, setTokensFound] = useState<CoinQuantity[]>([]);
  const [network, setNetwork] = useState<string>("")

  // Get initial network url from fuel wallet and listen for changes
  useNetwork(network, setNetwork, predicateAddress, setTokensFound);

  return (
    <>
    <header className="App-header">
      <Heading as="h1">OTC Swap</Heading>
    </header>

    <div className="App-main">
      <OfferInput setPredicateAddress={setPredicateAddress} setTokensFound={setTokensFound} network={network}/>
      <PredicateInfo predicateAddress={predicateAddress} tokensFound={tokensFound}/>
    </div>

    <div className="App-provider">
      <p> Connected to network: {network} </p>        
    </div>

    </>

  );
}
export default App;
