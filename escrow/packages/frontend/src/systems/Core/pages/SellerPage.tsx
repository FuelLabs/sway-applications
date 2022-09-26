import { useAtomValue } from "jotai";
import { useState } from "react";
import type { ChangeEvent } from "react";
import { useQueryClient } from "react-query";
import { Button, Card, Flex } from "@fuel-ui/react";
import { bn } from "fuels";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom, walletIndexAtom } from "../jotai";
import { useSellerEscrows } from "../hooks/useSellerEscrows";
import { useContract } from "../hooks/useContract";
import { ArbiterInputContainer } from "../components/ArbiterInputContainer";
import { useReturnDeposit } from "../hooks/useReturnDeposit";
import { EscrowInfo } from "../components/EscrowInfo";
import { useProposeArbiter } from "../hooks/useProposeArbiter";

export default function SellerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const sellerEscrows = useSellerEscrows();
  const returnDepositMutation = useReturnDeposit({ escrowId: bn(0) });

  // TODO DRY for repeated code in CreateEscrow.tsx
  const [arbiter, setArbiter] = useState("");
  const [arbiterAsset, setArbiterAsset] = useState("");
  const [arbiterFee, setArbiterFee] = useState("");

  const proposeArbiterMutation = useProposeArbiter(
    { arbiterAddress: arbiter,
      arbiterAsset,
      arbiterFee,
      escrowId: bn(0),
      setArbiterAddress: setArbiter,
      setArbiterAsset,
      setArbiterFee
    });


const handleArbiterAddressChange = (event: ChangeEvent<HTMLInputElement>) => {
  const newArbiter = event.target.value;
  setArbiter(newArbiter);
}

const handleArbiterAssetChange = (event: ChangeEvent<HTMLInputElement>) => {
  const newAssetId = event.target.value;
  setArbiterAsset(newAssetId);
}

const handleArbiterFeeChange = (event: ChangeEvent<HTMLInputElement>) => {
  const newFee = event.target.value;
  setArbiterFee(newFee);
}

const handleTakePayment = (escrowId: bigint) => {

}

const handleWithdrawCollateral = (escrowId: bigint) => {

}

return (
  <Layout>
    <Flex direction="column" justify="center">
      <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
        <CreateEscrow />
        {showBalances && <ShowBalances />}
      </Flex>
      <Flex justify="center">
        <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
          <Card.Header>
            Seller Escrows
          </Card.Header>
          {(!!sellerEscrows && sellerEscrows.length > 0)
            ? <>
              <EscrowInfo
                escrows={sellerEscrows}
              />

              {!!sellerEscrows[0].state.Pending &&
                <Card.Footer justify="space-evenly">
                  <ArbiterInputContainer
                    onArbiterAddressChange={handleArbiterAddressChange}
                    onAssetIdChange={handleArbiterAssetChange}
                    onFeeChange={handleArbiterFeeChange}
                    arbiterAddress={arbiter}
                    asset={arbiterAsset}
                    feeAmount={arbiterFee}
                  />
                  <Button onPress={() => proposeArbiterMutation.mutate()}>
                    Propose Arbiter
                  </Button>
                </Card.Footer>
              }

              {!!sellerEscrows[0].state.Pending &&
                <Card.Footer justify="space-evenly">
                  <Button onPress={() => returnDepositMutation.mutate()}>
                    Return Deposit
                  </Button>
                  <Button onPress={() => handleTakePayment(BigInt(0))}>
                    Take Payment
                  </Button>
                  <Button onPress={() => handleWithdrawCollateral(BigInt(0))}>
                    Withdraw Collateral
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
                Seller has no escrows
              </Card.Body>
            </>
          }

        </Card>
      </Flex>
    </Flex>
  </Layout>
);
}
