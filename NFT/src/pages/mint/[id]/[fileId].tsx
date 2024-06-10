import { useRouter } from "next/router";

import { GATEWAY_URL } from "@/lib";
import { Box, Stack, Typography } from "@mui/material";
import { Button } from "@/components/Button";
import { useTotalAssets } from "@/hooks/useTotalAssets";
import { useMint } from "@/hooks/useMint";
import toast from "react-hot-toast";

export default function Mint() {
  const router = useRouter();

  const contractId = router.query.nftContractId as string;
  const { totalAssets } = useTotalAssets(contractId);
  const mint = useMint();

  console.log(`contractId`, contractId);

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
            if (totalAssets) {
              mint.mutate({ totalAssets: totalAssets.toNumber(), contractId });
            } else {
              toast.error(`Cannot mint if total assets is ${totalAssets}`);
            }
          }}
          className="w-48"
        >
          Mint
        </Button>
      </Stack>
    </Box>
  );
}
