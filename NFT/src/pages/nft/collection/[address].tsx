import { NFTCard } from "@/components/NFTCard";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { getTruncatedAddress } from "@/utils/address";
import { Grid, Stack, Typography } from "@mui/material";
import { useRouter } from "next/router";

export default function Address() {
  const router = useRouter();

  // The filter expects a value so we pass in an impossible wallet address
  // in the case the user is disconnected
  const { nftData } = useGetNFTData({
    keyvalues: {
      minter: {
        value: router.query.address as string,
        op: "eq",
      },
    },
  });

  return (
    <>
      {nftData.length ? (
        <Stack spacing={2}>
          <Typography variant="h5" className="text-white font-sans">
            {getTruncatedAddress(router.query.address as string)} NFTs
          </Typography>
          <Grid container spacing={2}>
            {nftData.map((nftDatum) => {
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
                    showDescription
                  />
                </Grid>
              );
            })}
          </Grid>
        </Stack>
      ) : (
        <Typography className="text-white font-sans">No NFTs found</Typography>
      )}
    </>
  );
}
