import { ButtonLink, toast } from "@fuel-ui/react";
import { useEffect, useState } from "react"

export const WalletState = () => {
  const [connected, setConnection] = useState("Connect")

  useEffect(() => {
    async function main() {
      // If I don't update the state here too I'll need to refresh the page.
      // That being said, it doesn't matter what I set the state to in handleWalletConnection()
      // because this will overwrite it
      const isConnected = await window.fuel.isConnected();
      if (!isConnected) {
        setConnection("Connect");
      } else {
        setConnection("Disconnect");
      }

      let state = (document.getElementById("walletState") as HTMLLinkElement);
      state.innerText = connected;
    }
    main();
  }, [connected]);

  async function handleWalletConnection() {
    const isConnected = await window.fuel.isConnected();
    if (!isConnected) {
      await fuel.connect();
      toast.success("Connected!", { duration: 4000 });
      setConnection("Disconnect");
    } else {
      await fuel.disconnect();
      toast.success("Disconnected!", { duration: 4000 });
      setConnection("Connect");
    }
  }

  return (
    <ButtonLink href="#" onClick={handleWalletConnection} id="walletState" css={{ color: 'black', fontWeight: 'bolder' }}>
    </ButtonLink>
  );
}
