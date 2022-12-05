import { Card, Stack, Button } from "@fuel-ui/react";
import { Address, BN } from "fuels";
import { useState } from "react";

import { AssetAmountInput } from "~/systems/Core/components/AssetAmountInput";
import { ContractIdOutput } from "~/types/contracts/AuctionContractAbi";
import { AuctionAssetInput } from "~/types/contracts/EnglishAuctionAbi";
import { useBid } from "../hooks/useBid";

interface PlaceBidProps {
    auctionId: BN;
    auctionAssetAddress: ContractIdOutput;
};

export const PlaceBid = ({ auctionId, auctionAssetAddress }: PlaceBidProps) => {
    const [assetAmount, setAssetAmount] = useState("");
    const auctionAsset: AuctionAssetInput = { TokenAsset: { amount: assetAmount, asset_id: { value: auctionAssetAddress.toString() } }};
    const bidMutation = useBid({ auctionId, auctionAsset });

    return (
        <Card>
            <Card.Body>
                <Stack gap="$4">
                    <AssetAmountInput
                        assetAmountLabel="Place Bid"
                        assetAmountValue={assetAmount}
                        objKey="placeBidAmount"
                        onChange={setAssetAmount}
                    />
                    <Button onPress={() => bidMutation.mutate()}>Bid on Auction</Button>
                </Stack>
            </Card.Body>
        </Card>
    );
};