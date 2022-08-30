import { useAtomValue } from "jotai";
import { Button, Card, Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";
import { useSellerEscrows } from "../hooks/useSellerEscrows";
import { formatValue } from "../utils/helpers";
import { DECIMAL_PLACES } from "@/config";

export default function SellerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const sellerEscrows = useSellerEscrows();

  return (
    <Layout>
      <Flex direction="column" justify="center">
        <Flex justify="center">
          <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
            {(!!sellerEscrows && sellerEscrows.length > 0)
              ? <>
                <Card.Header>
                  Seller Escrows
                </Card.Header>
                <Card.Body>
                  <div>{`Arbiter: ${!!sellerEscrows[0].arbiter.address.Address ? sellerEscrows[0].arbiter.address.Address?.value : sellerEscrows[0].arbiter.address.ContractId!.value}`}</div>
                  <div>{`Arbiter Asset: ${sellerEscrows[0].arbiter.asset.value}`}</div>
                  <div>{`Arbiter Fee: ${formatValue(sellerEscrows[0].arbiter.fee_amount, DECIMAL_PLACES)}`}</div>
                  {sellerEscrows[0].assets.map((assetInfo) => {
                    return <>
                      <div>{`Asset Id: ${assetInfo.id.value}`}</div>
                      <div>{`Asset Amount: ${formatValue(assetInfo.amount, DECIMAL_PLACES)}`}</div>
                    </>
                  })}
                  <div>{`Buyer: ${!!sellerEscrows[0].buyer.address.Address ? sellerEscrows[0].buyer.address.Address?.value : sellerEscrows[0].buyer.address.ContractId?.value}`}</div>
                  <div>{`Buyer Desposit Asset: ${!!sellerEscrows[0].buyer.asset.None ? "None" : sellerEscrows[0].buyer.asset.Some?.value}`}</div>
                  <div>{`Buyer Deposit Amount: ${formatValue(sellerEscrows[0].buyer.deposited_amount, DECIMAL_PLACES)}`}</div>
                  <div>{`Deadline: ${sellerEscrows[0].deadline.toString()}`}</div>
                  <div>{`Disputed: ${sellerEscrows[0].disputed}`}</div>
                  <div>{`State: ${!!sellerEscrows[0].state.Pending ? "Pending" : "Completed"}`}</div>
                </Card.Body>
                <Card.Footer justify="space-evenly">
                  <Button>
                    Propose Arbiter
                  </Button>
                  <Button>
                    Return Deposit
                  </Button>
                  <Button>
                    Take Payment
                  </Button>
                  <Button>
                    Withdraw Collateral
                  </Button>
                </Card.Footer>
                <Card.Footer direction="row-reverse" gap="$4">
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
