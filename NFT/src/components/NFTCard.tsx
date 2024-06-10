import { Card, CardContent, Typography, CardActionArea } from "@mui/material";
import { useRouter } from "next/router";
import { GATEWAY_URL } from "@/lib";

type NFTCardProps = {
  cid: string;
  fileCid: string;
  nftName: string;
  nftDescription: string;
  nftContractId: string;
};

export const NFTCard = ({
  cid,
  fileCid,
  nftName,
  nftDescription,
  nftContractId
}: NFTCardProps) => {
  const router = useRouter();

  return (
    <Card>
      <CardActionArea
        onClick={() => {
          router.push(`/mint/${cid}/${fileCid}?nftName=${nftName}&nftDescription=${nftDescription}&nftContractId=${nftContractId}`);
        }}
      >
        <img src={`${GATEWAY_URL}/ipfs/${cid}/${fileCid}`} />
        <CardContent>
          <Typography variant="h5">{nftName}</Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
