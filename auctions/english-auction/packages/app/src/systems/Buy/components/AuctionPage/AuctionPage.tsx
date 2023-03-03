import { Stack, Card } from "@fuel-ui/react";
import type { BN } from "fuels";

import { PlaceBid } from "../PlaceBid";

import { AuctionAssetInfo } from "./AuctionAssetInfo";
import { AuctionEndActions } from "./AuctionEndActions";
import { AuctionEndInfo } from "./AuctionEndInfo";
import { AuctionIdentityInfo } from "./AuctionIdentityInfo";

import type { AuctionOutput } from "~/types/contracts/AuctionContractAbi";

interface AuctionPageProps {
  currentAuction: AuctionOutput;
  auctionId: BN;
}

export const AuctionPage = ({
  currentAuction,
  auctionId,
}: AuctionPageProps) => {
  const isSellAssetNFT = Boolean(currentAuction?.sell_asset.NFTAsset);
  const isBidAssetNFT = Boolean(currentAuction?.bid_asset.NFTAsset);

  let sellAsset;
  let sellAssetAmount;
  if (isSellAssetNFT) {
    sellAsset = currentAuction.sell_asset.NFTAsset!;
    sellAssetAmount = "1";
  } else {
    sellAsset = currentAuction.sell_asset.TokenAsset!;
    sellAssetAmount = currentAuction.sell_asset.TokenAsset!.amount.format()!;
  }

  let bidAsset;
  let bidAssetAmount;
  if (isBidAssetNFT) {
    bidAsset = currentAuction.bid_asset.NFTAsset!;
    bidAssetAmount = currentAuction.highest_bidder ? "1" : "0";
  } else {
    bidAsset = currentAuction.bid_asset.TokenAsset!;
    bidAssetAmount = currentAuction.bid_asset.TokenAsset!.amount.format();
  }

  const initialPrice = isBidAssetNFT
    ? "1"
    : currentAuction.initial_price.format()!;

  return (
    <Card css={{ alignSelf: "flex-start" }}>
      <Stack>
        <AuctionAssetInfo
          sellAsset={sellAsset}
          sellAssetAmount={sellAssetAmount}
          isSellAssetNFT={isSellAssetNFT}
          bidAsset={bidAsset}
          bidAssetAmount={bidAssetAmount}
          isBidAssetNFT={isBidAssetNFT}
          initialPrice={initialPrice}
        />

        {!currentAuction.state.Closed && (
          <PlaceBid
            auctionId={auctionId}
            auctionAsset={currentAuction.bid_asset}
            seller={currentAuction.seller!}
          />
        )}

        <AuctionIdentityInfo
          sellerAddress={
            currentAuction.seller.Address?.value ||
            currentAuction.seller.ContractId!.value
          }
          highestBidder={currentAuction.highest_bidder}
        />

        <AuctionEndInfo
          auctionState={currentAuction.state}
          endBlock={currentAuction.end_block}
        />

        <AuctionEndActions
          seller={currentAuction.seller}
          auctionId={auctionId}
          auctionState={currentAuction.state}
          bidAsset={currentAuction.bid_asset}
          sellAsset={currentAuction.sell_asset}
        />
      </Stack>
    </Card>
  );
};
