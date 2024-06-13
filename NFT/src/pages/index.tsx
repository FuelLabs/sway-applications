import { Grid, Stack, Typography } from "@mui/material";
import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";

export default function Home() {
  const { nftData } = useGetNFTData();

  return (
    <Stack alignItems="flex-start" width="stretch" spacing={3}>
      <Typography className="text-white font-sans" variant="h3">Latest NFTs</Typography>
      <Grid container spacing={2}>
        {nftData?.map((nftDatum) => {
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
              />
            </Grid>
          );
        })}
      </Grid>
    </Stack>
  );
}
