import { Flex, Heading } from "@fuel-ui/react";
import type { BN } from "fuels";
import { useEffect, useState } from "react";

import { useLatestBlockHeight } from "~/systems/Core/hooks/useLatestBlockHeight";

interface EndBlockProps {
  endBlock: BN;
  onChange: (expired: boolean) => void;
}

export const EndBlock = ({ endBlock, onChange }: EndBlockProps) => {
  const [curBlocksAway, setCurBlocksAway] = useState<BN>();
  const latestBlockHeight = useLatestBlockHeight();

  const calcBlocksAway = (blockHeight0: BN, blockHeight1: BN) => {
    const result = blockHeight0.sub(blockHeight1);
    if (blockHeight0.lt(blockHeight1)) {
      return result.ineg();
    }
    return result;
  };

  useEffect(() => {
    const blocksAway = calcBlocksAway(endBlock, latestBlockHeight);
    setCurBlocksAway(blocksAway);
    onChange(blocksAway.isNeg()!);
  }, [latestBlockHeight]);

  const endText = curBlocksAway?.isNeg()
    ? "Auction Ended"
    : `Auction ends in ${curBlocksAway?.toString()} blocks at block height ${endBlock.toString()}`;

  return (
    <Flex>
      <Heading as="h5" css={{ marginLeft: "$5" }}>
        {endText}
      </Heading>
    </Flex>
  );
};
