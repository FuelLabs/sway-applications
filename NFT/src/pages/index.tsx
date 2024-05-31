import contractId from "@/contract-types/contract-ids.json";
import {
  Grid,
} from "@mui/material";
import { NFTCard } from "@/components/NFTCard";
import { useGetNFTData } from "@/hooks/useGetNFTData";

export default function Home() {
  const { nftData } = useGetNFTData();

  return (
    <Grid container spacing={2}>
      {[...Array(10)].map(() => {
        return (
          <Grid xs={3}>
            <NFTCard cid={nftData[0]?.ipfs_pin_hash} />
          </Grid>
        );
      })}
      {/* <UploadButton setCid={setCid} />
      {cid && <Files cid={cid} />} */}
    </Grid>
  );
}
