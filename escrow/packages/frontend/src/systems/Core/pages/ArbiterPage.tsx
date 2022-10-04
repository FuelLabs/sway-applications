import { Flex, Card, Button, Input, Dropdown } from "@fuel-ui/react";
import { bn } from "fuels";
import { useAtomValue } from "jotai";
import type { ChangeEvent } from "react";
import React, { useState } from "react";

import { EscrowInfo } from "../components/EscrowInfo";
import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { useArbiterEscrows } from "../hooks/useArbiterEscrows";
import { useResolveDispute } from "../hooks/useResolveDispute";
import { showBalancesAtom } from "../jotai";

export default function BuyerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const arbiterEscrows = useArbiterEscrows();
  const [arbiterPayment, setArbiterPayment] = useState("");
  const [favoredUser, setFavoredUser] = useState("");
  const resolveDisputeMutation = useResolveDispute({
    escrowId: bn(0),
    arbiterPayment,
    favoredUser,
  });

  const handleAssetAmountChange = (event: ChangeEvent<HTMLInputElement>) => {
    setArbiterPayment(event.target.value);
  };

  const handleUserChange = (event: React.Key) => {
    setFavoredUser(event.toString());
  };

  return (
    <Layout>
      <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
        <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
          <Card.Header>Arbiter Escrows</Card.Header>
          {!!arbiterEscrows && arbiterEscrows.length > 0 ? (
            <>
              <EscrowInfo escrows={arbiterEscrows} />

              {arbiterEscrows[0].disputed && arbiterEscrows[0].state.Pending && (
                <Card.Footer justify="space-evenly">
                  <Input>
                    <Input.Number
                      aria-label="Resolve arbiter fee input"
                      placeholder="Fee Amount"
                      value={arbiterPayment}
                      inputMode="decimal"
                      onChange={(e) => handleAssetAmountChange(e)}
                    />
                  </Input>
                  <Dropdown>
                    <Dropdown.Trigger>
                      <Button>User to favor</Button>
                    </Dropdown.Trigger>
                    <Dropdown.Menu
                      autoFocus
                      aria-label="Actions"
                      onAction={(e) => handleUserChange(e)}
                    >
                      <Dropdown.MenuItem
                        key={`${arbiterEscrows[0].seller.address.Address?.value}`}
                        textValue="Seller"
                      >
                        Seller
                      </Dropdown.MenuItem>
                      <Dropdown.MenuItem
                        key={`${arbiterEscrows[0].buyer.address.Address?.value}`}
                        textValue="Buyer"
                      >
                        Buyer
                      </Dropdown.MenuItem>
                    </Dropdown.Menu>
                  </Dropdown>
                  <Button
                    aria-label="Resolve dispute"
                    onPress={() => resolveDisputeMutation.mutate()}
                  >
                    Resolve Dispute
                  </Button>
                </Card.Footer>
              )}
              <Card.Footer direction="row-reverse" gap="$4">
                <Button leftIcon="DotsThree">Show all escrows</Button>
              </Card.Footer>
            </>
          ) : (
            <>
              <Card.Body>Arbiter has no escrows</Card.Body>
            </>
          )}
        </Card>

        {showBalances && <ShowBalances />}
      </Flex>
    </Layout>
  );
}
