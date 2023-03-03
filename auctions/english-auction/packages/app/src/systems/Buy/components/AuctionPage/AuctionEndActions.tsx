import type { BN } from "fuels";

import { CancelAuctionButton } from "../CancelAuctionButton";
import { WithdrawButton } from "../WithdrawButton";

import type {
  AuctionAssetOutput,
  StateInput,
  IdentityOutput,
} from "~/types/contracts/AuctionContractAbi";

interface AuctionEndActionsProps {
  auctionState: StateInput;
  auctionId: BN;
  seller: IdentityOutput;
  bidAsset: AuctionAssetOutput;
  sellAsset: AuctionAssetOutput;
}

export const AuctionEndActions = ({
  auctionState,
  auctionId,
  seller,
  bidAsset,
  sellAsset,
}: AuctionEndActionsProps) => {
  return (
    <>
      {auctionState.Open ? (
        <CancelAuctionButton index={auctionId.toNumber()} seller={seller} />
      ) : (
        <WithdrawButton
          auctionId={auctionId}
          seller={seller}
          bidAsset={bidAsset}
          sellAsset={sellAsset}
        />
      )}
    </>
  );
};
