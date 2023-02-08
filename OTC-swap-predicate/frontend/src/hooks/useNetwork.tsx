import { useEffect } from "react";
import type { FuelProviderConfig } from "@fuel-wallet/types";
import useProvider from "./useProvider";
import useFuel from "./useFuel";
import getTokenBalance from "../utils/predicateBalance";
import { Address, CoinQuantity } from "fuels";

function useNetwork(network: string, setNetwork: React.Dispatch<React.SetStateAction<string>>, predicateAddress: Address, setTokensFound: React.Dispatch<React.SetStateAction<CoinQuantity[]>>) {
    let {provider} = useProvider();
      
    if (provider !== undefined && network === "") {
      setNetwork(provider!.url);
    }
  
    const [fuel] = useFuel();

    const handleNetworkChange = async (network: FuelProviderConfig ) => {
        setNetwork(network.url);

        // On network change, re-check for tokens belonging to the current predicate address
        // If error, set tokensFound to empty array
        console.log("handleNetworkChange");
        let tokens: CoinQuantity[];
        try {
          tokens = await getTokenBalance(predicateAddress, network.url);
        }
        catch(e){
          console.log("Network change error: " + e);
          tokens = [];
        }
        setTokensFound(tokens);
    };
  
    useEffect(() => {

        fuel?.on("network", handleNetworkChange);
    
        return () => {
            fuel?.off("network", handleNetworkChange);
        };
    }, [fuel]);
  }

export default useNetwork;