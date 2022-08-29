import { useAtomValue } from "jotai";
import { Button, Card, Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";
import { useSellerEscrows } from "../hooks/useSellerEscrows";

export default function SellerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const { result: sellerEscrows, loading } = useSellerEscrows();
  console.log("seller escrows: ", sellerEscrows);

  return (
    <Layout>
      <Flex direction="column" justify="center">
        <Flex justify="center">
          <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
            {(sellerEscrows && sellerEscrows.length > 0)
              ? <>
                <Card.Header>
                  Seller Escrows
                </Card.Header>
                <Card.Body>
                  {sellerEscrows!.map((escrowId, i) => (
                    <>
                      {escrowId}
                    </>
                  ))}
                </Card.Body>
                <Card.Footer direction="row-reverse">
                  <Button leftIcon="DotsThree">
                    Show all escrows
                  </Button>
                </Card.Footer>
              </>
              : <>
                <Card.Header>
                  Seller Escrows
                </Card.Header>
                <Card.Body>
                  Seller has no escrows
                </Card.Body>
              </>
            }

          </Card>
        </Flex>
        <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
          <CreateEscrow />
          {showBalances && <ShowBalances />}
        </Flex>
      </Flex>
    </Layout>
  );
}
