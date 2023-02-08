import { useEffect } from "react";
import { useProvider } from "../hooks/useProvider";
import { useFuel } from "../hooks/useFuel";
import type { FuelProviderConfig } from "@fuel-wallet/types";


export function Network(network: string, setNetwork: React.Dispatch<React.SetStateAction<string>>) {
    let {provider} = useProvider();
      
    if (provider !== undefined && network === "") {
      setNetwork(provider!.url);
    }
  
    const [fuel] = useFuel();

    const handleNetworkChange = (network: FuelProviderConfig ) => {
        setNetwork(network.url);
    };
  
    useEffect(() => {

        fuel?.on("network", handleNetworkChange);
    
        return () => {
            fuel?.off("network", handleNetworkChange);
        };
    }, [fuel]);
  }
