import { Card, Stack, Button, toast, Text, Icon, Flex } from "@fuel-ui/react";
import { Address, BN, Wallet } from "fuels";
import { useEffect, useState } from "react";

import { AssetAmountInput } from "~/systems/Core/components/AssetAmountInput";
import { useWallet } from "~/systems/Core/hooks/useWallet";
import { ContractIdOutput, IdentityOutput } from "~/types/contracts/AuctionContractAbi";
import { AuctionAssetInput } from "~/types/contracts/EnglishAuctionAbi";
import { useBid } from "../hooks/useBid";

interface PlaceBidProps {
    auctionId: BN;
    auctionAssetAddress: ContractIdOutput;
    seller: IdentityOutput;
};

export const PlaceBid = ({ auctionId, auctionAssetAddress, seller }: PlaceBidProps) => {
    const [assetAmount, setAssetAmount] = useState("");
    const [identityOutput, setIdentityOutput] = useState<string>();
    const auctionAsset: AuctionAssetInput = { TokenAsset: { amount: assetAmount, asset_id: { value: auctionAssetAddress.toString() } } };
    const bidMutation = useBid({ auctionId, auctionAsset });
    const wallet = useWallet();

    if (!wallet) toast.error("Wallet not detected");

    useEffect(() => {
        const result = wallet?.address.toHexString();
        setIdentityOutput(result);
    }, [wallet]);

    return (
        <Card>
            <Card.Body>
                {identityOutput === seller.Address?.value ?
                    <Text>
                        <Flex gap="$4">
                            <Icon icon="X" color="tomato10"/>
                            Error sellers cannot bid on their own auctions.  Change your wallet to bid on the auction.
                        </Flex>
                    </Text> :
                    <Stack gap="$4">
                        <AssetAmountInput
                            assetAmountLabel="Place Bid"
                            assetAmountValue={assetAmount}
                            objKey="placeBidAmount"
                            onChange={setAssetAmount}
                        />
                        <Button onPress={() => bidMutation.mutate()}>Bid on Auction</Button>
                    </Stack>
                }
            </Card.Body>
        </Card>
    );
};