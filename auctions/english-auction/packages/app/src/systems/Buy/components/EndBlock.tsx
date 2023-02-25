import { Flex, Heading, toast } from "@fuel-ui/react";
import type { BN } from "fuels";
import { bn } from "fuels";
import { useEffect, useState } from "react";

import { useLatestBlockHeight } from "~/systems/Core/hooks/useLatestBlockHeight";

interface EndBlockProps {
  endBlock: BN;
  onChange: (expired: boolean) => void;
}

export const EndBlock = ({ endBlock, onChange }: EndBlockProps) => {
  const [endText, setEndText] = useState<string>("");
  const { latestBlockHeight, isLoading, isError } = useLatestBlockHeight();

  if (!isLoading && (!latestBlockHeight || isError)) {
    toast.error("Could not fetch latest block height");
  }

  const calcBlocksAway = (blockHeight0: BN, blockHeight1: BN): BN => {
    const result = blockHeight0.sub(blockHeight1);
    if (blockHeight0.lt(blockHeight1)) {
      return bn(result.ineg());
    }
    return result;
  };

  useEffect(() => {
    const blocksAway: BN = latestBlockHeight
      ? calcBlocksAway(endBlock, latestBlockHeight)
      : bn(0);
    const newEndText = blocksAway?.isNeg()
      ? "Auction Ended"
      : `Auction ends in ${blocksAway?.toString()} blocks at block height ${endBlock.toString()}`;
    setEndText(newEndText);
    onChange(blocksAway.isNeg()!);
  }, [latestBlockHeight]);

  return (
    <Flex>
      {isLoading ? (
        <div>Loading...</div>
      ) : (
        <Heading as="h5" css={{ marginLeft: "$5" }}>
          {endText}
        </Heading>
      )}
    </Flex>
  );
};
