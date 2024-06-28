import { Text } from "@/components/Text";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { TESTNET_FAUCET_LINK } from "@/lib";

export default function Faucet() {
  const { wallet } = useActiveWallet();

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
