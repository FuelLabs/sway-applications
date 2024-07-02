import { Grid, Stack } from "@mui/material";
import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Text } from "@/components/Text";
import { QueryClient, dehydrate } from "@tanstack/react-query";
import { NFTQueryKeys } from "@/queryKeys";
import { getNFTMetadata } from "../api/files/[filter]";
import { NFTGrid } from "@/components/NFTGrid";

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
  const { nftData, isLoading } = useGetNFTData();

  return (
    <NFTGrid
      isLoading={isLoading}
      nftData={nftData}
      title={<Text variant="h3">Latest NFTs</Text>}
    />
  );
}
