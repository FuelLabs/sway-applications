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
        return blockHeight0.sub(blockHeight1);
    };

    useEffect(() => {
        const blocksAway = calcBlocksAway(endBlock, latestBlockHeight);
        setCurBlocksAway(blocksAway);
    }, [latestBlockHeight]);


    return (
        <>
            <Heading as="h5">{`Auction ends in ${curBlocksAway?.toString()} blocks at block height ${endBlock.toString()}`}</Heading>
        </>
    );
}