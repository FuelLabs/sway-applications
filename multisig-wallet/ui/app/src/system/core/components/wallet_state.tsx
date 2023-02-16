import { ButtonLink, toast } from "@fuel-ui/react";
import { useEffect, useState } from "react"

export const WalletState = () => {
  const [connected, setConnection] = useState("Connect")

  useEffect(() => {
    async function main() {
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
    const isConnected = await window.fuel.isConnected();
    if (!isConnected) {
      await fuel.connect();
      toast.success("Connected!", { duration: 4000 });
    } else {
      await fuel.disconnect();
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
