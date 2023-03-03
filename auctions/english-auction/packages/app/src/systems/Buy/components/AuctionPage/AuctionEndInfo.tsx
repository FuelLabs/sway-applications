import { Heading } from "@fuel-ui/react";
import type { BN } from "fuels";

import { EndBlock } from "../EndBlock";

import type { StateInput } from "~/types/contracts/AuctionContractAbi";

interface AuctionEndInfoProps {
  auctionState: StateInput;
  endBlock: BN;
}

export const AuctionEndInfo = ({
  auctionState,
  endBlock,
}: AuctionEndInfoProps) => {
  return (
    <>
      {auctionState.Closed ? (
        <Heading as="h5" css={{ marginLeft: "$5" }}>
          Auction Closed
        </Heading>
      ) : (
        <EndBlock endBlock={endBlock} onChange={() => {}} />
      )}
    </>
  );
};
