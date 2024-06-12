import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Grid, Typography } from "@mui/material";
import { useRouter } from "next/router";

export default function Collection() {
  const router = useRouter();

  const walletAddress = router.query.address as string;
  const { nftData } = useGetNFTData({
    keyvalues: {
      minter: {
        value: walletAddress,
        op: "eq",
      },
    },
  });

  console.log(`nftData`, nftData);

  return (
    <Grid container spacing={2}>
      {nftData ? nftData.map((nftDatum) => {
        return (
          <Grid xs={3}>
            <NFTCard
              cid={nftDatum.ipfs_pin_hash}
              fileCid={nftDatum.metadata?.name || ""}
              nftName={nftDatum.metadata.keyvalues?.nftName || ""}
              nftDescription={nftDatum.metadata.keyvalues?.nftDescription || ""}
              nftSubId={nftDatum.metadata.keyvalues?.nftSubId || ""}
            />
          </Grid>
        );
      }) : (
        <Typography>No NFTs found</Typography>
      )}
    </Grid>
  );
}
