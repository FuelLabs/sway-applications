import { useGetNFTData } from "@/hooks/useGetNFTData";
import { getTruncatedAddress } from "@/utils/address";
import { Box, IconButton, Stack, Tooltip } from "@mui/material";
import { useRouter } from "next/router";
import OpenInNewIcon from "@mui/icons-material/OpenInNew";

import { Text } from "@/components/Text";
import { NFTGrid } from "@/components/NFTGrid";

export default function Address() {
  const router = useRouter();

  // The filter expects a value so we pass in an impossible wallet address
  // in the case the user is disconnected
  const { nftData, isLoading } = useGetNFTData({
    keyvalues: {
      minter: {
        value: router.query.address as string,
        op: "eq",
      },
    },
  });

  const accountAddress = router.query.address as string;

  return (
    <NFTGrid
      isLoading={isLoading}
      nftData={nftData}
      title={
        <Stack>
          <Text variant="h5">NFTs in Account</Text>
          <Box display="flex" alignItems="center">
            <Text variant="h5">{getTruncatedAddress(accountAddress)}</Text>
            <Tooltip title="Open in Explorer">
              <IconButton
                className="text-white"
                target="_blank"
                href={`https://fuel-explorer.vercel.app/account/${accountAddress}`}
              >
                <OpenInNewIcon />
              </IconButton>
            </Tooltip>
          </Box>
        </Stack>
      }
    />
  );
}
