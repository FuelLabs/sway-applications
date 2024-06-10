import { Grid } from "@mui/material";
import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";

export default function Home() {
  const { nftData } = useGetNFTData();

  return (
    <Grid container spacing={2}>
      {nftData?.map((nftDatum) => {
        return (
          <Grid xs={3}>
            <NFTCard
              cid={nftDatum.ipfs_pin_hash}
              fileCid={nftDatum.metadata.name || ""}
              nftName={nftDatum.metadata.keyvalues.nftName}
              nftDescription={nftDatum.metadata.keyvalues?.nftDescription}
              nftContractId={nftDatum.metadata.keyvalues?.nftContractId}
            />
          </Grid>
        );
      })}
    </Grid>
  );
}
