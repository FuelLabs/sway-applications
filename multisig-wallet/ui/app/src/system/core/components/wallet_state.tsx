import { ButtonLink, toast } from "@fuel-ui/react";
import { useEffect, useState } from "react"
import { useFuel } from "../hooks";

export const WalletState = () => {
  const [connected, setConnection] = useState("Connect")
  const fuel = useFuel()

  useEffect(() => {
    async function main() {
      // const isConnected = await fuel!.isConnected();
      const isConnected = await window.fuel.isConnected();
      if (!isConnected) {
        setConnection("Connect");
      } else {
        setConnection("Disconnect");
      }
    }
    main();
  }, [connected]);

  async function handleWalletConnection() {
    // const isConnected = await fuel!.isConnected();
    const isConnected = await window.fuel.isConnected();
    if (!isConnected) {
      await window.fuel.connect();
      // await fuel!.connect();
      toast.success("Connected!", { duration: 4000 });
    } else {
      await window.fuel.disconnect();
      // await fuel!.disconnect();
      toast.success("Disconnected!", { duration: 4000 });
    }

    // trigger useEffect
    setConnection("");
  }

  return (
    <ButtonLink onClick={handleWalletConnection} css={{ color: 'black', fontWeight: 'bolder' }}>
      {connected}
    </ButtonLink>
  );
}
