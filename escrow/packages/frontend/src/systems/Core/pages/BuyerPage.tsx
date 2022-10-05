import { Flex, Card, Button } from "@fuel-ui/react";
import { useAtomValue } from "jotai";

import { Deposit } from "../components/Deposit";
import { EscrowInfo } from "../components/EscrowInfo";
import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { useAcceptArbiter } from "../hooks/useAcceptArbiter";
import { useArbiterProposal } from "../hooks/useArbiterProposal";
import { useBuyerEscrows } from "../hooks/useBuyerEscrows";
import { useDispute } from "../hooks/useDispute";
import { useTransferToSeller } from "../hooks/useTransferToSeller";
import { showBalancesAtom } from "../jotai";
import { parseToFormattedNumber } from "../utils/math";

export default function BuyerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const { buyerEscrows, buyerEscrowIds } = useBuyerEscrows();
  const transferToSellerMutation = useTransferToSeller({ escrowId: buyerEscrowIds![0] });
  const disputeMutation = useDispute({ escrowId: buyerEscrowIds![0] });
  const acceptArbiterMutation = useAcceptArbiter({ escrowId: buyerEscrowIds[0] });
  const arbiterProposal = useArbiterProposal(buyerEscrowIds[0]);

  return (
    <Layout>
      <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
        <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
          <Card.Header>Buyer Escrows</Card.Header>
          {!!buyerEscrows && buyerEscrows.length > 0 ? (
            <>
              <EscrowInfo escrows={buyerEscrows} />

              {!buyerEscrows[0].buyer.asset && (
                <Card.Footer justify="space-evenly">
                  <Deposit escrowId={buyerEscrowIds[0]} />
                </Card.Footer>
              )}

              {!!buyerEscrows[0].state.Pending && !!arbiterProposal && (
                <Card.Footer justify="space-evenly">
                  <div>{`Arbiter: ${arbiterProposal?.address.Address?.value.slice(
                    0,
                    4
                  )}...${arbiterProposal.address.Address?.value.slice(
                    -4
                  )}`}</div>
                  <div>{`Fee: ${parseToFormattedNumber(
                    arbiterProposal?.fee_amount.toString()
                  )}`}</div>
                  <div>{`Asset: ${arbiterProposal?.asset.value.slice(
                    0,
                    4
                  )}...${arbiterProposal.asset.value.slice(-4)}`}</div>
                  <Button
                    aria-label="Accept arbiter button"
                    onPress={() => acceptArbiterMutation!.mutate()}
                  >
                    Accept Arbiter
                  </Button>
                </Card.Footer>
              )}

              {!!buyerEscrows[0].state.Pending && (
                <Card.Footer justify="space-evenly">
                  <Button
                    aria-label="Transfer to seller"
                    onPress={() => transferToSellerMutation.mutate()}
                  >
                    Transfer To Seller
                  </Button>
                  {!buyerEscrows[0].disputed && (
                    <Button
                      aria-label="Dispute"
                      onPress={() => disputeMutation.mutate()}
                    >
                      Dispute
                    </Button>
                  )}
                </Card.Footer>
              )}

              <Card.Footer direction="row-reverse" gap="$4">
                <Button leftIcon="DotsThree">Show all escrows</Button>
              </Card.Footer>
            </>
          ) : (
            <>
              <Card.Body>Buyer has no Escrows</Card.Body>
            </>
          )}
        </Card>
        {/* <Deposit /> */}
        {showBalances && <ShowBalances />}
      </Flex>
    </Layout>
  );
}
