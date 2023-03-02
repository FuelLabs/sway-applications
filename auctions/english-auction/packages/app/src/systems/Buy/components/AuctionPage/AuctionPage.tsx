import { Stack } from "@fuel-ui/react";

import { AuctionAssetInfo } from "./AuctionAssetInfo";

import type { AuctionOutput } from "~/types/contracts/AuctionContractAbi";

interface AuctionPageProps {
    currentAuction: AuctionOutput;
}

export const AuctionPage = ({
currentAuction
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
    </Stack>
  );
};
