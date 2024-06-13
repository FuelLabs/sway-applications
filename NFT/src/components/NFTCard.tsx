import { Card, CardContent, Typography, CardActionArea } from "@mui/material";
import { useRouter } from "next/router";
import { GATEWAY_URL } from "@/lib";

type NFTCardProps = {
  cid: string;
  fileCid: string;
  nftName: string;
  nftDescription: string;
  nftSubId: string;
};

export const NFTCard = ({
  cid,
  fileCid,
  nftName,
  nftDescription,
  nftSubId
}: NFTCardProps) => {
  const router = useRouter();

  return (
    <Card variant="outlined" sx={{ padding: "18px", backgroundColor: "black", borderColor: "#1e1e1e", borderWidth: "3px" }}>
      <CardActionArea
        onClick={() => {
          router.push(`/mint/${cid}/${fileCid}?nftName=${nftName}&nftDescription=${nftDescription}&nftSubId=${nftSubId}`);
        }}
      >
        <img src={`${GATEWAY_URL}/ipfs/${cid}/${fileCid}`} width="350px" height="350px" />
        <CardContent sx={{ paddingBottom: "0px" }}>
          <Typography color="white" variant="h5">{nftName}</Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
