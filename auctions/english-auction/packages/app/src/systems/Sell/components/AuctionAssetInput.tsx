import { Button, Dropdown, Icon, Input } from "@fuel-ui/react";
import { DECIMAL_UNITS } from "fuels";
import { useState } from "react";

export const AuctionAssetInput = () => {
  const [tokenType, setTokenType] = useState("token");

  const handleTokenTypeSelection = (newTokenType: string) => {
    setTokenType(newTokenType);
  };

  return (
    <>
      <Input css={{ alignSelf: "stretch" }}>
        {tokenType === "token" ? (
          <Input.Number
            allowedDecimalSeparators={[".", ","]}
            allowNegative={false}
            autoComplete="off"
            inputMode="decimal"
            decimalScale={DECIMAL_UNITS}
            placeholder="Bid Asset Amount"
            thousandSeparator={false}
          />
        ) : (
          <Input.Number
            allowNegative={false}
            autoComplete="off"
            inputMode="numeric"
            placeholder="Bid Asset Token Id"
          />
        )}
      </Input>
      <Input css={{ alignSelf: "stretch" }}>
        <Input.Field placeholder="Bid Asset Id" />
      </Input>
      <Dropdown>
        <Dropdown.Trigger>
          <Button>Choose Asset Type</Button>
        </Dropdown.Trigger>
        <Dropdown.Menu
          autoFocus
          onAction={(e) => handleTokenTypeSelection(e.toString())}
        >
          <Dropdown.MenuItem key="token" textValue="Token">
            <Icon icon="Coin" />
            Token
          </Dropdown.MenuItem>
          <Dropdown.MenuItem key="nft" textValue="NFT">
            <Icon icon="Image" />
            NFT
          </Dropdown.MenuItem>
        </Dropdown.Menu>
      </Dropdown>
    </>
  );
};
