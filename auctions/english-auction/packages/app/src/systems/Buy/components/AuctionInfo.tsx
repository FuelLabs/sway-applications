import { OptionalAuctionOutput } from "~/types/contracts/EnglishAuctionAbi";

import { Stack, Flex, Button, Heading } from "@fuel-ui/react";
import { bn } from "fuels";

import { AssetOutput, AssetIdOutput } from "~/systems/Core/components";
import { PlaceBid, EndBlock } from "~/systems/Buy/components";
import { getSlicedAddress } from "~/systems/Core/utils";
import { useState } from "react";
import { useCancelAuction } from "../hooks/useCancelAuctaion";
import { useBid } from "../hooks/useBid";
import { CancelAuctionButton } from "./CancelAuctionButton";

interface AuctionInfoProps {
    auctions: OptionalAuctionOutput[];
}

export const AuctionInfo = ({ auctions }: AuctionInfoProps) => {

    const auctionInfo = auctions?.map((auction, index) => {
        const [auctionExpired, setAuctionExpired] = useState(false);

        return (
            <Stack>
                <Flex>
                    <AssetOutput
                        assetId={auction?.sell_asset.TokenAsset?.asset_id.value!}
                        assetAmount={auction?.sell_asset.TokenAsset?.amount.format()!}
                        heading="Selling"
                    />

                    <AssetOutput
                        assetId={auction?.bid_asset.TokenAsset?.asset_id.value!}
                        assetAmount={auction?.bid_asset.TokenAsset?.amount.format()!}
                        heading="Highest Bid"
                    />

                    <AssetOutput
                        assetId={auction?.bid_asset.TokenAsset?.asset_id.value!}
                        assetAmount={auction?.initial_price.format()!}
                        heading="Initial Price"
                    />
                </Flex>

                {(!auctionExpired && !auction?.state.Closed) &&
                    <PlaceBid
                        auctionId={bn(index)}
                        auctionAssetAddress={auction?.bid_asset.TokenAsset?.asset_id!}
                        seller={auction?.seller!}
                    />
                }

                <Flex>
                    <AssetIdOutput
                        assetId={getSlicedAddress(auction?.seller.Address?.value!)}
                        heading="Seller"
                    />

                    <AssetIdOutput
                        assetId={(auction?.highest_bidder?.Address?.value && getSlicedAddress(auction?.highest_bidder?.Address?.value)) || "None"}
                        heading="Highest Bidder"
                    />
                </Flex>

                {auction?.state.Closed ?
                    <Heading as="h5">Auction Closed</Heading> :
                    <EndBlock endBlock={auction!.end_block} onChange={setAuctionExpired} />
                }
                {(!auctionExpired && !auction?.state.Closed) &&
                    <CancelAuctionButton index={index} seller={auction?.seller!} />
                }
            </Stack>
        );
    });

    return (
        <>
            {auctionInfo}
        </>
    );
};