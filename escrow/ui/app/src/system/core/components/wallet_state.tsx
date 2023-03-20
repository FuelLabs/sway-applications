import { Button } from "@fuel-ui/react";
import { useEffect, useState } from "react";
import { useFuel } from "../hooks";

export const WalletState = () => {
  const [connected, setConnection] = useState("Connect");
  const fuel = useFuel();

  useEffect(() => {
    if (!fuel) return;

    async function main() {
      const isConnected = await fuel!.isConnected();
      if (!isConnected) {
        setConnection("Connect");
      } else {
        setConnection("Disconnect");
      }
    }
    main();
  }, [connected, fuel]);

  async function handleWalletConnection() {
    const isConnected = await fuel!.isConnected();
    if (!isConnected) {
      await fuel!.connect();
    } else {
      await fuel!.disconnect();
    }

    // trigger useEffect
    setConnection("");

    // User needs to refresh without this to be able to interact with the contract / UI
    window.location.reload();
  }

  return (
    <Button
      onPress={handleWalletConnection}
      variant="ghost"
      css={{
        background: "rgb(34 196 53)",
        color: "$blackA12",
        fontWeight: "$semibold",
        border: "1px solid black",
      }}
    >
      {connected}
    </Button>
  );
};
