import { useAtomValue } from "jotai";
import { Button, Card, Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";
import { useWallet } from "../context/AppContext";
import { useContract } from "../hooks/useContract";
import { useSellerEscrows } from "../hooks/useSellerEscrows";

export default function SellerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const wallet = useWallet();
  const contract = useContract();
  const { data: sellerEscrows } = useSellerEscrows();

  return (
    <Layout>
      <Flex direction="column" justify="center">
        <Flex justify="center">
          <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
            <Card.Header>
              Seller Escrows
            </Card.Header>
            <Card.Body>
              {sellerEscrows?.map((escrow, i) => (

              ))}
            </Card.Body>
            <Card.Footer direction="row-reverse">
                <Button leftIcon="DotsThree">
                  Show all escrows
                </Button>
            </Card.Footer>
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
