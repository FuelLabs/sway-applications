import { NFTCard } from "@/components/NFTCard";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Grid, Stack, Typography } from "@mui/material";
import { useRouter } from "next/router";

export default function Home() {
  const { isConnected, wallet } = useActiveWallet();

  // The filter expects a value so we pass in an impossible wallet address
  // in the case the user is disconnected
  const { nftData } = useGetNFTData({
    keyvalues: {
      minter: {
        value: wallet?.address.toB256() || "dud",
        op: "eq",
      },
    },
  });

  console.log(`nftData`, nftData);

  return (
    <>
      {nftData.length ? (
        <Stack spacing={2}>
          <Typography variant="h5" className="text-white font-sans">Your NFTs</Typography>
          <Grid container spacing={2}>
            {nftData.map((nftDatum) => {
              return (
                <Grid>
                  <NFTCard
                    cid={nftDatum.ipfs_pin_hash}
                    fileCid={nftDatum.metadata?.name || ""}
                    nftName={nftDatum.metadata.keyvalues?.nftName || ""}
                    nftDescription={
                      nftDatum.metadata.keyvalues?.nftDescription || ""
                    }
                    nftSubId={nftDatum.metadata.keyvalues?.nftSubId || ""}
                    showDescription
                  />
                </Grid>
              );
            })}
          </Grid>
        </Stack>
      ) : isConnected ? (
        <Typography className="text-white font-sans">No NFTs found</Typography>
      ) : (
        <Typography className="text-white font-sans">Please connect your wallet to view your NFTs.</Typography>
      )}
    </>
  );
}
