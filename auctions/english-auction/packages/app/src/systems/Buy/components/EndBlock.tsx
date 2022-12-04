import type { BN } from "fuels";
import { Heading } from "@fuel-ui/react";
import { useLatestBlockHeight } from "~/systems/Core/hooks/useLatestBlockHeight";
import { useEffect, useState } from "react";

interface EndBlockProps {
    endBlock: BN;
};

export const EndBlock = ({ endBlock }: EndBlockProps) => {
    const [curBlocksAway, setCurBlocksAway] = useState<BN>();
    const latestBlockHeight = useLatestBlockHeight();

    const calcBlocksAway = (blockHeight0: BN, blockHeight1: BN) => {
        const result = blockHeight0.sub(blockHeight1);
        if (blockHeight0.lt(blockHeight1)) {
            return result.ineg()
        }
        return result;
    };

    useEffect(() => {
        console.log("end block: ", endBlock.toString());
        console.log("latest block: ", latestBlockHeight?.toString());
        const blocksAway = calcBlocksAway(endBlock, latestBlockHeight);
        setCurBlocksAway(blocksAway);
    }, [latestBlockHeight]);

    const endText = curBlocksAway?.isNeg() ? 'Auction Ended' : `Auction ends in ${curBlocksAway?.toString()} blocks at block height ${endBlock.toString()}`;

    return (
        <>
            <Heading as="h5">{endText}</Heading>
        </>
    );
}