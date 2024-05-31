import { useRouter } from "next/router";

import { GATEWAY_URL } from "@/lib";
import { Box, Stack, Typography } from "@mui/material";
import { Button } from "@/components/Button";

export default function Mint() {
  const router = useRouter();

  return (
    <Box display="flex" justifyContent="space-between" width="50rem">
      <img src={`${GATEWAY_URL}/ipfs/${router.query.id}`} />
      <Stack width="300px" spacing={2}>
        <Typography variant="h5">NFT TITLE</Typography>
        <Typography>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et dolore magna aliqua.
        </Typography>
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
