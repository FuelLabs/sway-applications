import { useRouter } from "next/router";

import { GATEWAY_URL } from "@/lib";
import { Box, Stack, Typography } from "@mui/material";
import { Button } from "@/components/Button";
import { useTotalAssets } from "@/hooks/useTotalAssets";
import { useMint } from "@/hooks/useMint";
import toast from "react-hot-toast";

export default function Mint() {
  const router = useRouter();

  const subId = router.query.nftSubId as string;
  const nftName = router.query.nftName as string;
  const nftDescription = router.query.nftDescription as string;

  const mint = useMint();

  return (
    <Box display="flex" justifyContent="space-between" width="50rem">
      <img
        src={`${GATEWAY_URL}/ipfs/${router.query.id}/${router.query.fileId}`}
      />
      <Stack width="300px" spacing={2}>
        <Typography variant="h5">{nftName}</Typography>
        {router.query.nftDescription && (
          <Typography>{nftDescription}</Typography>
        )}
        <Button
          onClick={() => {
              mint.mutate({
                nftSubId: subId,
                cid: router.query.id as string,
                nftName,
                nftDescription
              });
          }}
          className="w-48"
        >
          Mint
        </Button>
      </Stack>
    </Box>
  );
}
