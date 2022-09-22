import { useAtomValue } from "jotai";
import { Flex, Card, Button, Input, Grid } from "@fuel-ui/react";
import toast from 'react-hot-toast';
import { useQueryClient } from "react-query";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom, walletIndexAtom } from "../jotai";
import { Deposit } from "../components/Deposit";
import { useBuyerEscrows } from "../hooks/useBuyerEscrows";
import { useContract } from "../hooks/useContract";
import { formatValue } from "../utils/helpers";
import { DECIMAL_PLACES } from "@/config";
import { useTransferToSeller } from "../hooks/useTransferToSeller";
import { useDispute } from "../hooks/useDispute";
import { EscrowInfo } from "../components/EscrowInfo";
import { useArbiterProposal } from "../hooks/useArbiterProposal";

export default function BuyerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const buyerEscrows = useBuyerEscrows();
  const contract = useContract();
  const transferToSellerMutation = useTransferToSeller({ escrowId: BigInt(0) });
  const disputeMutation = useDispute({ escrowId: BigInt(0) });
  const arbiterProposal = useArbiterProposal(BigInt(0));

  const handleAcceptArbiter = (escrowId: bigint) => {

  }

  return (
    <Layout>
      <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
        <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
          <Card.Header>
            Buyer Escrows
          </Card.Header>
          {(!!buyerEscrows && buyerEscrows.length > 0)
            ? <>
              <EscrowInfo escrows={buyerEscrows} />

              {!buyerEscrows[0].buyer.asset &&
                <Card.Footer>
                  <Deposit escrowId={BigInt(0)} />
                </Card.Footer>
              }

              {(!!buyerEscrows[0].state.Pending && arbiterProposal?.Some) &&
                <Card.Footer justify="space-evenly">
                    <div>{`Arbiter: ${arbiterProposal.Some?.address}`}</div>
                    <div>{`Fee: ${arbiterProposal.Some?.fee_amount}`}</div>
                    <div>{`Asset: ${arbiterProposal.Some?.asset}`}</div>
                </Card.Footer>

              }

              {!!buyerEscrows[0].state.Pending &&
                <Card.Footer justify="space-evenly">
                  <Button onPress={() => handleAcceptArbiter(BigInt(0))}>
                    Accept Arbiter
                  </Button>
                  <Button onPress={() => transferToSellerMutation.mutate()}>
                    Transfer To Seller
                  </Button>
                  <Button onPress={() => disputeMutation.mutate()}>
                    Dispute
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
                Buyer has no Escrows
              </Card.Body>
            </>
          }

        </Card>
        {/* <Deposit /> */}
        {showBalances && <ShowBalances />}
      </Flex>
    </Layout>
  );
}
