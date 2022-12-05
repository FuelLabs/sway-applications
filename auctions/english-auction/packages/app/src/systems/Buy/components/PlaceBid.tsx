import { Card, Stack, Button } from "@fuel-ui/react";

import { AssetAmountInput } from "~/systems/Core/components/AssetAmountInput";

export const PlaceBid = () => {

    return (
        <Card>
            <Card.Body>
                <Stack gap="$4">
                    <AssetAmountInput
                        assetAmountLabel="Place Bid"
                        assetAmountValue=""
                        objKey="placeBidAmount"
                        onChange={() => { console.log("here") }}
                    />
                    <Button>Bid on Auction</Button>
                </Stack>
            </Card.Body>
        </Card>
    );
};