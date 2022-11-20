import { Button, Card, Flex, Input, Stack } from "@fuel-ui/react";

import { AuctionAssetInput } from "./AuctionAssetInput";

export const CreateAuction = () => {
  return (
    <Flex justify="center">
      <Card>
        <Card.Header>Create Auction</Card.Header>
        <Stack css={{ width: "475px", margin: "10px", alignItems: "center" }}>
          <AuctionAssetInput
            placeholderAssetId="Bid Asset Id"
            placeholderTokenAmount="Bid Asset Amount"
            placeholderTokenId="Bid Token Id"
          />
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number inputMode="numeric" placeholder="Duration" />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Field placeholder="Initial Price" />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Field placeholder="Reserve Price" />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Field placeholder="Seller" />
          </Input>
          <AuctionAssetInput
            placeholderAssetId="Sell Asset Id"
            placeholderTokenAmount="Sell Asset Amount"
            placeholderTokenId="Sell Token Id"
          />
          <Button leftIcon="Plus">Create Auction</Button>
        </Stack>
      </Card>
    </Flex>
  );
};
