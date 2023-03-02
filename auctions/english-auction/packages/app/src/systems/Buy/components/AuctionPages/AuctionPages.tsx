import { Pagination, Heading, Stack } from "@fuel-ui/react";
import { useState } from "react";

import { AuctionPage } from "../AuctionPage/AuctionPage";

import type {
  AuctionOutput,
} from "~/types/contracts/AuctionContractAbi";
import type { Option } from "~/types/contracts/common";

interface AuctionPagesProps {
  auctions: Option<AuctionOutput>[];
}

export const AuctionPages = ({ auctions }: AuctionPagesProps) => {
  const [currentPageNumber, setCurrentPageNumber] = useState(1);

  if (auctions.length === 0) {
    return <Heading>No Auctions</Heading>;
  }

  const currentAuction = auctions[currentPageNumber - 1];

  if (!currentAuction) return;

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

  const initialPrice = isBidAssetNFT ? "1" : currentAuction.initial_price.format()!;

  return (
    <Stack>
      <AuctionPage
        sellAsset={sellAsset}
        sellAssetAmount={sellAssetAmount}
        isSellAssetNFT={isSellAssetNFT}
        bidAsset={bidAsset}
        bidAssetAmount={bidAssetAmount}
        isBidAssetNFT={isBidAssetNFT}
        initialPrice={initialPrice}
      />
      <Pagination
        pagesCount={auctions.length}
        onPageChange={setCurrentPageNumber}
      >
        <Pagination.Prev />
        <Pagination.Items />
        <Pagination.Next />
      </Pagination>
    </Stack>
  );
};
