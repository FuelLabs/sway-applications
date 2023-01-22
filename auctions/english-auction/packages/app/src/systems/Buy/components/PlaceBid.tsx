import { Card, Stack, Button, toast, Text, Icon, Flex } from "@fuel-ui/react";
import type { BN } from "fuels";
import { bn, DECIMAL_UNITS } from "fuels";
import { useEffect, useState } from "react";

import { useBid } from "../hooks/useBid";

import { AssetAmountInput } from "~/systems/Core/components/AssetAmountInput";
import { useWallet } from "~/systems/Core/hooks/useWallet";
import { NumericFormInput } from "~/systems/Sell/components/NumericFormInput";
import type {
  AuctionAssetOutput,
  IdentityOutput,
} from "~/types/contracts/AuctionContractAbi";

interface PlaceBidProps {
  auctionId: BN;
  auctionAsset: AuctionAssetOutput;
  seller: IdentityOutput;
}

export const PlaceBid = ({
  auctionId,
  auctionAsset,
  seller,
}: PlaceBidProps) => {
  const [assetAmount, setAssetAmount] = useState("");
  const [identityOutput, setIdentityOutput] = useState<string>();
  const bidMutation = useBid({
    auctionId,
    auctionAsset: !auctionAsset.NFTAsset
      ? {
          TokenAsset: {
            amount: bn.parseUnits(assetAmount, DECIMAL_UNITS),
            asset_id: { value: auctionAsset.TokenAsset!.asset_id.value },
          },
        }
      : {
          NFTAsset: {
            token_id: bn(assetAmount),
            asset_id: auctionAsset.NFTAsset.asset_id,
          },
        },
    setAssetAmount,
  });
  const { wallet } = useWallet();

  if (!wallet) toast.error("Wallet not detected");

  useEffect(() => {
    const result = wallet!.address.toHexString();
    setIdentityOutput(result);
  }, [wallet]);

  const isNFT = !!auctionAsset.NFTAsset;

  return (
    <Card>
      <Card.Body>
        {identityOutput === seller.Address?.value ? (
          <Text aria-label="Seller cannot bid">
            <Flex gap="$4">
              <Icon icon="X" color="tomato10" />
              Error sellers cannot bid on their own auctions. Change your wallet
              to bid on the auction.
            </Flex>
          </Text>
        ) : (
          <Stack gap="$4">
            {isNFT ? (
              <NumericFormInput
                formLabel="Enter NFT Token Id"
                formValue={assetAmount}
                objKey="placeBidAmount"
                onChange={(_, val) => setAssetAmount(val)}
              />
            ) : (
              <AssetAmountInput
                assetAmountLabel="Enter Bid Amount"
                assetAmountValue={assetAmount}
                objKey="placeBidAmount"
                onChange={(_, val) => setAssetAmount(val)}
              />
            )}

            <Button onPress={() => bidMutation.mutate()}>Bid on Auction</Button>
          </Stack>
        )}
      </Card.Body>
    </Card>
  );
};
