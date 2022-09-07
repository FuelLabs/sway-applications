import { useAtomValue } from "jotai";
import { Flex } from "@fuel-ui/react";
import { css } from "@fuel-ui/css";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";
import { Card, Button, Input } from "@fuel-ui/react";
import { useArbiterEscrows } from "../hooks/useArbiterEscrows";
import { EscrowInfo } from "../components/EscrowInfo";
import { useResolveDispute } from "../hooks/useResolveDispute";
import { useState, ChangeEvent } from "react";
import { Dropdown } from "../components/Dropdown";

export default function BuyerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const arbiterEscrows = useArbiterEscrows();
  const [arbiterPayment, setArbiterPayment] = useState("");
  const [favoredUser, setFavoredUser] = useState("");
  const resolveDisputeMutation = useResolveDispute({
    escrowId: BigInt(0),
    arbiterPayment,
    favoredUser
  });

  const handleAssetAmountChange = (event: ChangeEvent<HTMLInputElement>) => {
    setArbiterPayment(event.target.value);
  }

  const handleUserChange = (event: ChangeEvent<HTMLInputElement>) => {
    setFavoredUser(event.target.value);
  }

  return (
    <Layout>
      <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
        <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
          <Card.Header>
            Arbiter Escrows
          </Card.Header>
          {(!!arbiterEscrows && arbiterEscrows.length > 0)
            ? <>
              <EscrowInfo
                escrows={arbiterEscrows}
              />

              {(arbiterEscrows[0].disputed && arbiterEscrows[0].state.Pending) &&
                <Card.Footer justify="space-evenly">
                  <Input>
                    <Input.Number
                      placeholder="Fee Amount"
                      value={arbiterPayment}
                      inputMode="decimal"
                      onChange={(e) => handleAssetAmountChange(e)}
                    />
                  </Input>
                  <Dropdown className={dropDownStyle()}>
                    Seller
                    Buyer
                  </Dropdown>
                  <Button onPress={() => resolveDisputeMutation.mutate()}>
                    Resolve Dispute
                  </Button>
                </Card.Footer>
              }
              <Card.Footer direction="row-reverse" gap="$4">
                <Button leftIcon="DotsThree">
                  Show all escrows
                </Button>
              </Card.Footer>
            </>
            : <>
              <Card.Body>
                Arbiter has no escrows
              </Card.Body>
            </>
          }
        </Card>

        {showBalances && <ShowBalances />}
      </Flex>
    </Layout>
  );
}

const dropDownStyle = css({
  bg: "$accent9",
  color: "$gray1",
  textSize: "base",
  font: "$sans",
  cursor: "pointer",
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  border: "1px solid transparent",
  borderRadius: "$lg",
});