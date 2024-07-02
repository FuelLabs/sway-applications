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

const MAX_INITIAL_DESCRIPTION = 256;

const NFTDescription = ({ nftDescription }: { nftDescription: string }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const showReadMore = nftDescription.length > MAX_INITIAL_DESCRIPTION;

  return (
    <>
      {showReadMore ? (
        <Stack spacing={2}>
          {isExpanded ? (
            <>
              <Text>{nftDescription}</Text>
              <Button onClick={() => setIsExpanded(false)}>Show less</Button>
            </>
          ) : (
            <>
              <Text>{nftDescription.slice(0, MAX_INITIAL_DESCRIPTION)}...</Text>
              <Button onClick={() => setIsExpanded(true)}>Show more</Button>
            </>
          )}
        </Stack>
      ) : (
        <Text>{nftDescription}</Text>
      )}
    </>
  );
};

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
    <Stack
      justifyContent="center"
      spacing={3}
      className={clsx(
        "gradient-border",
        "h-full",
        "rounded-xl",
        "bg-gradient-to-b",
        "from-zinc-900",
        "to-zinc-950/80",
        "px-2",
        "py-8",
        "sm:w-3/4",
        "md:w-1/2"
      )}
    >
      <Box display="flex" alignSelf="center">
        <NFTImage
          src={`${GATEWAY_URL}/ipfs/${router.query.id}/${router.query.fileId}`}
          className="w-80 h-80 lg:w-96 lg:h-96"
        />
      </Box>
      <Stack className="px-4" spacing={2}>
        <Text variant="h5">{nftName}</Text>
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
            {mint.isPending ? "Minting..." : "Mint"}
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
        {router.query.nftDescription && (
          <NFTDescription nftDescription={nftDescription} />
        )}
      </Stack>
    </Stack>
  );
}
