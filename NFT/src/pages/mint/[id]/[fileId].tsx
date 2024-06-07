import { useRouter } from "next/router";

import { GATEWAY_URL } from "@/lib";
import { Box, Stack, Typography } from "@mui/material";
import { Button } from "@/components/Button";

export default function Mint() {
  const router = useRouter();

  console.log(`router.query`, router.query);

  return (
    <Box display="flex" justifyContent="space-between" width="50rem">
      <img
        src={`${GATEWAY_URL}/ipfs/${router.query.id}/${router.query.fileId}`}
      />
      <Stack width="300px" spacing={2}>
        <Typography variant="h5">{router.query.nftName}</Typography>
        {router.query.nftDescription && (
          <Typography>{router.query.nftDescription}</Typography>
        )}
        <Button
          onClick={() => {
            console.log("mint");
          }}
          className="w-48"
        >
          Mint
        </Button>
      </Stack>
    </Box>
  );
}
