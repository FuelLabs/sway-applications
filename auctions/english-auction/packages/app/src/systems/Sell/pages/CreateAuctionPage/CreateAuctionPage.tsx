import { Button, Icon, Stack } from "@fuel-ui/react";
import { DECIMAL_UNITS, bn } from "fuels";

import { CreateAuctionForm } from "../../components/CreateAuctionForm";
import { useCreateAuction } from "../../hooks/useCreateAuction";
import { useCreateAuctionForm } from "../../hooks/useCreateAuctionForm";
import type { CreateAuctionFormValues } from "../../hooks/useCreateAuctionForm";

import { MainLayout, useAssets, useWallet } from "~/systems/Core";

export const CreateAuctionPage = () => {
  const form = useCreateAuctionForm();
  const { wallet, isLoading, isError } = useWallet();
  if (isError) throw new Error("Error: no wallet connected");
  const assets = useAssets() || [];
  const createAuctionMutation = useCreateAuction();

  function onSubmit(formValues: CreateAuctionFormValues) {
    createAuctionMutation.mutate({
      sellerAddress: formValues.sellerAddress,
      sellAsset: !formValues.isSellAssetNft
        ? {
            TokenAsset: {
              amount: bn.parseUnits(formValues.sellAssetAmount, DECIMAL_UNITS),
              asset_id: { value: formValues.sellAssetId },
            },
          }
        : {
            NFTAsset: {
              token_id: formValues.sellNFTTokenId,
              asset_id: { value: formValues.sellNFTAssetId },
            },
          },
      initialPrice: bn.parseUnits(formValues.initialPrice, DECIMAL_UNITS),
      reservePrice: formValues.hasReservePrice
        ? formValues.reservePrice
        : undefined,
      bidAsset: !formValues.isBidAssetNft
        ? {
            TokenAsset: {
              amount: bn(0),
              asset_id: { value: formValues.bidAssetId },
            },
          }
        : {
            NFTAsset: {
              token_id: bn(0),
              asset_id: { value: formValues.bidNFTAssetId },
            },
          },
      duration: formValues.duration,
    });
    form.reset();
  }

  // TODO feat: add loaders to components
  return (
    <MainLayout>
      {isLoading ? (
        <div>Loading...</div>
      ) : (
        <form onSubmit={form.handleSubmit(onSubmit)}>
          <Stack css={{ width: "475px" }}>
            <CreateAuctionForm
              form={form}
              walletAddress={wallet!.address.toString()!}
              assets={assets}
            />
            <Button
              type="submit"
              color="accent"
              isDisabled={!form.formState.isValid}
              isLoading={isLoading}
              leftIcon={Icon.is("Plus")}
            >
              Create Auction
            </Button>
          </Stack>
        </form>
      )}
    </MainLayout>
  );
};
