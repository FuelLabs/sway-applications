import {
  Card,
  CardContent,
  Typography,
  CardActionArea,
} from "@mui/material";
import { useRouter } from "next/router";
import { GATEWAY_URL } from "@/lib";

type NFTCardProps = {
  cid: string;
};

export const NFTCard = ({ cid }: NFTCardProps) => {
    const router = useRouter();

  return (
    <Card>
      <CardActionArea
        onClick={() => {
          router.push(`/mint/${cid}`);
        }}
      >
        <img src={`${GATEWAY_URL}/ipfs/${cid}`} />
        <CardContent>
          <Typography variant="h5">NFT NAME</Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
