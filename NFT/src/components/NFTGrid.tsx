import { NFTData } from "@/hooks/useGetNFTData";
import { Box, Grid, Skeleton, Stack } from "@mui/material";
import { ReactNode } from "react";
import { NFTCard } from "./NFTCard";

type NFTGridProps = {
  isLoading?: boolean;
  title: ReactNode;
  nftData: NFTData[];
};

const NFTGridLoader = () => {
  return (
    <>
      {Array(6)
        .fill(0)
        .map(() => {
          return (
            <Grid item xs={12} sm={6} md={4}>
              <Skeleton
                variant="rectangular"
                height="250px"
                className="bg-gray-900"
              />
            </Grid>
          );
        })}
    </>
  );
};

export const NFTGrid = ({ isLoading, title, nftData }: NFTGridProps) => {
  return (
    <Stack spacing={2} alignItems="flex-start" className="w-full">
      <Box display="flex" alignSelf="center">
        {title}
      </Box>
      <Grid container spacing={2} className="-ml-4">
        {isLoading ? (
          <NFTGridLoader />
        ) : (
          nftData.map((nftDatum) => {
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
          })
        )}
      </Grid>
    </Stack>
  );
};
