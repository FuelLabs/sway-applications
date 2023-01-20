import { Stack, Flex, Card, Heading } from "@fuel-ui/react";
import { bn } from "fuels";
import { useState } from "react";

import { CancelAuctionButton } from "./CancelAuctionButton";
import { WithdrawButton } from "./WithdrawButton";

import { PlaceBid, EndBlock } from "~/systems/Buy/components";
import { AssetOutput, AssetIdOutput } from "~/systems/Core/components";
import { getSlicedAddress } from "~/systems/Core/utils";
import type { AuctionOutput } from "~/types/contracts/AuctionContractAbi";
import type { Option } from "~/types/contracts/common";

interface AuctionInfoProps {
  auctions: Option<AuctionOutput>[];
}

export const AuctionInfo = ({ auctions }: AuctionInfoProps) => {
  if (auctions.length === 0) {
    return <Heading>No Auctions</Heading>;
  }
  const auctionInfo = auctions.map((auction, index) => {
    if (!auction) return;
    const [auctionExpired, setAuctionExpired] = useState(false);

    const isSellAssetNft = !!auction.sell_asset.NFTAsset;
    const isBidAssetNft = !!auction.bid_asset.NFTAsset;

    const sellAsset = isSellAssetNft
      ? auction.sell_asset.NFTAsset!
      : auction.sell_asset.TokenAsset!;
    const sellAssetAmount = isSellAssetNft
      ? "1"
      : auction.sell_asset.TokenAsset!.amount!.format();

    const bidAsset = isBidAssetNft
      ? auction.bid_asset.NFTAsset!
      : auction.bid_asset.TokenAsset!;
    const bidAssetAmount = isBidAssetNft // eslint-disable-line no-nested-ternary
      ? auction.highest_bidder
        ? "1"
        : "0"
      : auction.bid_asset.TokenAsset!.amount.format();

    const initialPrice = isBidAssetNft ? "1" : auction.initial_price.format()!;

    return (
      <Card key={index}>
        <Stack>
          <Flex>
            <AssetOutput
              assetId={sellAsset.asset_id.value}
              assetAmount={sellAssetAmount}
              heading="Selling"
              isNFT={isSellAssetNft}
            />

            <AssetOutput
              assetId={bidAsset.asset_id.value}
              assetAmount={bidAssetAmount}
              heading="Highest Bid"
              isNFT={isBidAssetNft}
            />

            <AssetOutput
              assetId={bidAsset.asset_id.value}
              assetAmount={initialPrice}
              heading="Initial Price"
              isNFT={isBidAssetNft}
            />
          </Flex>

          {!auctionExpired && !auction.state.Closed && (
            <PlaceBid
              auctionId={bn(index)}
              auctionAsset={auction.bid_asset}
              seller={auction.seller!}
            />
          )}

          <Flex>
            {/* TODO show bech32 address */}
            <AssetIdOutput
              assetId={getSlicedAddress(auction.seller.Address!.value!)}
              heading="Seller"
            />

            <AssetIdOutput
              assetId={
                (auction.highest_bidder &&
                  getSlicedAddress(auction.highest_bidder.Address!.value)) ||
                "None"
              }
              heading="Highest Bidder"
            />
          </Flex>

          {auction?.state.Closed ? (
            <Heading as="h5" css={{ marginLeft: "$5" }}>
              Auction Closed
            </Heading>
          ) : (
            <EndBlock
              endBlock={auction!.end_block}
              onChange={setAuctionExpired}
            />
          )}
          {!auctionExpired && !auction?.state.Closed ? (
            <CancelAuctionButton index={index} seller={auction!.seller!} />
          ) : (
            <WithdrawButton auctionId={bn(index)} />
          )}
        </Stack>
      </Card>
    );
  });

  return <>{auctionInfo}</>;
};
