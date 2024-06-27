import { Grid, Stack } from "@mui/material";
import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Text } from "@/components/Text";

export default function Home() {
  const { nftData, isPending } = useGetNFTData();

  return (
    <Stack alignItems="flex-start" width="stretch" spacing={3}>
      <Text variant="h3">Latest NFTs</Text>
      {isPending ? (
        <Text>Loading...</Text>
      ) : (
        <Grid container spacing={2}>
          {nftData?.map((nftDatum) => {
            return (
              <Grid item>
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
      )}
    </Stack>
  );
}
