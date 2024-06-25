import { useRouter } from "next/router";

import { GATEWAY_URL } from "@/lib";
import { Box, Stack } from "@mui/material";
import { Button } from "@/components/Button";
import { useMint } from "@/hooks/useMint";
import { useTotalSupply } from "@/hooks/useTotalSupply";
import clsx from "clsx";
import { NFTImage } from "@/components/NFTImage";
import { useActiveWallet } from "@/hooks/useActiveWallet";
import { useGetNFTData } from "@/hooks/useGetNFTData";
import { Link } from "@/components/Link";
import { getTruncatedAddress } from "@/utils/address";
import { Text } from "@/components/Text";
import { useEffect, useState } from "react";

export default function Mint() {
  const router = useRouter();
  const [minterAddress, setMinterAddress] = useState("");

  const subId = (router.query.nftSubId || "dud") as string;
  const nftName = router.query.nftName as string;
  const nftDescription = router.query.nftDescription as string;

  const { totalSupply } = useTotalSupply(subId);
  const { isConnected } = useActiveWallet();

  const { nftData } = useGetNFTData({
    keyvalues: {
      nftSubId: {
        value: subId,
        op: "eq",
      },
    },
  });

  useEffect(() => {
    if (nftData.length && nftData[0].metadata.keyvalues.minter) {
      setMinterAddress(nftData[0].metadata.keyvalues.minter);
    }
  }, [nftData, nftData.length]);

  const mint = useMint();

  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent="space-around"
      width="40rem"
      className={clsx(
        "gradient-border",
        "h-full",
        "rounded-xl",
        "bg-gradient-to-b",
        "from-zinc-900",
        "to-zinc-950/80",
        "px-2",
        "py-8"
      )}
    >
      <NFTImage
        src={`${GATEWAY_URL}/ipfs/${router.query.id}/${router.query.fileId}`}
      />
      <Stack width="200px" spacing={2}>
        <Text variant="h5">
          {nftName}
        </Text>
        {router.query.nftDescription && (
          <Text>
            {nftDescription}
          </Text>
        )}
        {!totalSupply ? (
          <Button
            onClick={() => {
              mint.mutate({
                nftSubId: subId,
                cid: router.query.id as string,
                nftName,
                nftDescription,
              });
            }}
            className="w-48"
            disabled={!isConnected || mint.isPending}
          >
            {mint.isPending ? "Loading..." : "Mint"}
          </Button>
        ) : minterAddress ? (
          <Text>
            NFT minted by{" "}
            <Link href={`/nft/collection/${minterAddress}`}>
              {getTruncatedAddress(
                nftData[0].metadata.keyvalues.minter as string
              )}
            </Link>
          </Text>
        ) : (
          <Text>Loading...</Text>
        )}
      </Stack>
    </Box>
  );
}
