import { Grid, Stack } from "@mui/material";
import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Text } from "@/components/Text";
import { QueryClient, dehydrate } from "@tanstack/react-query";
import { NFTQueryKeys } from "@/queryKeys";
import { getNFTMetadata } from "../api/files/[filter]";

export async function getStaticProps() {
  const queryClient = new QueryClient();

  await queryClient.prefetchQuery({
    queryKey: [NFTQueryKeys.nftData],
    queryFn: () => {
      return getNFTMetadata();
    },
  });

  return {
    props: {
      dehydratedState: dehydrate(queryClient),
    },
  };
}

export default function Home() {
  const { nftData, isPending } = useGetNFTData();

  return (
    <Stack alignItems="flex-start" width="stretch" spacing={2}>
      <Text variant="h3">Latest NFTs</Text>
      {isPending ? (
        <Text>Loading...</Text>
      ) : (
        <Grid container spacing={2} className="-ml-4">
          {nftData?.map((nftDatum) => {
            return (
              <Grid item xs={12} sm={4}>
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
