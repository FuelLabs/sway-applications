import { Card, CardContent, CardActionArea } from "@mui/material";
import { useRouter } from "next/router";
import { GATEWAY_URL } from "@/lib";
import { NFTImage } from "./NFTImage";
import { Text } from "./Text";

type NFTCardProps = {
  cid: string;
  fileCid: string;
  nftName: string;
  nftDescription: string;
  nftSubId: string;
  showDescription?: boolean;
};

export const NFTCard = ({
  cid,
  fileCid,
  nftName,
  nftDescription,
  nftSubId,
  showDescription,
}: NFTCardProps) => {
  const router = useRouter();

  return (
    <Card
      variant="outlined"
      sx={{
        padding: "18px",
        backgroundColor: "black",
        borderColor: "#1e1e1e",
        borderWidth: "3px",
        height: "420px",
      }}
    >
      <CardActionArea
        onClick={() => {
          router.push(
            `/nft/mint/${cid}/${fileCid}?nftName=${nftName}&nftDescription=${nftDescription}&nftSubId=${nftSubId}`
          );
        }}
      >
        <NFTImage src={`${GATEWAY_URL}/ipfs/${cid}/${fileCid}`} />
        <CardContent sx={{ paddingBottom: "0px" }}>
          <Text variant="h5">
            {nftName}
          </Text>
          {showDescription && (
            <Text>
              {nftDescription}
            </Text>
          )}
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
