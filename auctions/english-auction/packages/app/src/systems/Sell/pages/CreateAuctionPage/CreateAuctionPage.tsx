import { Button, Icon, Stack } from "@fuel-ui/react";

import { CreateAuctionForm } from "../../components/CreateAuctionForm";
import { useCreateAuctionForm } from "../../hooks/useCreateAuctionForm";
import type { CreateAuctionFormValues } from "../../hooks/useCreateAuctionForm";

import { MainLayout, useAssets, useWallet } from "~/systems/Core";

export const CreateAuctionPage = () => {
  const form = useCreateAuctionForm();
  const { wallet, isLoading, isError } = useWallet();
  if (isError) throw new Error("Error: no wallet connected");
  const assets = useAssets() || [];

  function onSubmit(_: CreateAuctionFormValues) {
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
