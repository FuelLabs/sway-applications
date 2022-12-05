import { OptionalAuctionOutput } from "~/types/contracts/EnglishAuctionAbi";

import { Stack, Flex, Button, Heading } from "@fuel-ui/react";
import { bn } from "fuels";

import { AssetOutput, AssetIdOutput } from "~/systems/Core/components";
import { PlaceBid, EndBlock } from "~/systems/Buy/components";
import { getSlicedAddress } from "~/systems/Core/utils";
import { useState } from "react";
import { useCanceAuction } from "../hooks/useCancelAuctaion";

interface AuctionInfoProps {
    auctions: OptionalAuctionOutput[];
}

export const AuctionInfo = ({ auctions }: AuctionInfoProps) => {

    const auctionInfo = auctions?.map((auction, index) => {
        const [auctionExpired, setAuctionExpired] = useState(false);
        const cancelAuctionMutation = useCanceAuction({ auctionId: bn(index) });

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

                {(!auctionExpired && !auction?.state.Closed) && <PlaceBid />}

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

                {auction?.state.Closed ? <Heading as="h5">Auction Closed</Heading> : <EndBlock endBlock={auction!.end_block} onChange={setAuctionExpired} />}
                {(!auctionExpired && !auction?.state.Closed) && <Button onPress={() => cancelAuctionMutation.mutate()} css={{ minWidth: "100%" }}>Cancel Auction</Button>}
            </Stack>
        );
    });

    return (
        <>
            {auctionInfo}
        </>
    );
};