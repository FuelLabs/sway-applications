import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { getTruncatedAddress } from "@/utils/address";
import { Grid, Stack } from "@mui/material";
import { useRouter } from "next/router";

import { Text } from "@/components/Text";

export default function Address() {
  const router = useRouter();

  // The filter expects a value so we pass in an impossible wallet address
  // in the case the user is disconnected
  const { nftData, isPending } = useGetNFTData({
    keyvalues: {
      minter: {
        value: router.query.address as string,
        op: "eq",
      },
    },
  });

  return (
    <>
      {isPending ? (
        <Text>Loading...</Text>
      ) : nftData.length ? (
        <Stack spacing={2} className="w-full">
          <Text variant="h5">
            {getTruncatedAddress(router.query.address as string)} NFTs
          </Text>
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
      ) : (
        <Text>No NFTs found</Text>
      )}
    </>
  );
}
