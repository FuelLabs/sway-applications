import { Text } from "@/components/Text";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { TESTNET_FAUCET_LINK } from "@/lib";
import { useEffect } from "react";

export default function Faucet() {
  const { wallet, refetchBalnce } = useActiveWallet();

  useEffect(() => {
    const interval = setInterval(refetchBalnce, 500);
    return () => clearInterval(interval);
  }, [])

  if (!wallet) return <Text>Please connect wallet to faucet funds.</Text>;

  return (
    <iframe
      src={`${TESTNET_FAUCET_LINK}?address=${wallet.address.toAddress()}`}
      width="100%"
      height="700px"
      allowFullScreen
    >
      hello
    </iframe>
  );
}
