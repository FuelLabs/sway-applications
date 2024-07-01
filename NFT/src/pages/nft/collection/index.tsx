import { NFTCard } from "@/components/NFTCard";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Grid, Stack } from "@mui/material";

import { Text } from "@/components/Text";

export default function Home() {
  const { isConnected, wallet, isPending: isWalletPending } = useActiveWallet();

  // The filter expects a value so we pass in an impossible wallet address
  // in the case the user is disconnected
  const { nftData, isPending: isNFTDataPending } = useGetNFTData({
    keyvalues: {
      minter: {
        value: wallet?.address.toB256() || "dud",
        op: "eq",
      },
    },
  });

  const isPending = isNFTDataPending || isWalletPending;

  return (
    <>
      {isPending ? (
        <Text>Loading...</Text>
      ) : nftData.length ? (
        <Stack spacing={2} className="w-full">
          <Text variant="h5">Your NFTs</Text>
          <Grid container spacing={2} className="-ml-4">
            {nftData.map((nftDatum) => {
              return (
                <Grid item xs={12} sm={6} md={4}>
                  <NFTCard
                    cid={nftDatum.ipfs_pin_hash}
                    fileCid={nftDatum.metadata?.name || ""}
                    nftName={nftDatum.metadata.keyvalues?.nftName || ""}
                    nftDescription={
                      nftDatum.metadata.keyvalues?.nftDescription || ""
                    }
                    nftSubId={nftDatum.metadata.keyvalues?.nftSubId || ""}
                  />
                </Grid>
              );
            })}
          </Grid>
        </Stack>
      ) : isConnected ? (
        <Text>No NFTs found</Text>
      ) : (
        <Text>Please connect your wallet to view your NFTs.</Text>
      )}
    </>
  );
}
