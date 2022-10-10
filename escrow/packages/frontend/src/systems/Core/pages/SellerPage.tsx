import { Button, Card, Flex } from "@fuel-ui/react";
import { useAtomValue } from "jotai";
import { useState } from "react";
import type { ChangeEvent } from "react";

import { ArbiterInputContainer } from "../components/ArbiterInputContainer";
import { CreateEscrow } from "../components/CreateEscrow";
import { EscrowInfo } from "../components/EscrowInfo";
import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { useProposeArbiter } from "../hooks/useProposeArbiter";
import { useReturnDeposit } from "../hooks/useReturnDeposit";
import { useSellerEscrows } from "../hooks/useSellerEscrows";
import { useTakePayment } from "../hooks/useTakePayment";
import { useWithdrawCollateral } from "../hooks/useWithdrawCollateral";
import { showBalancesAtom } from "../jotai";

export default function SellerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const { sellerEscrows, sellerEscrowIds } = useSellerEscrows();
  const returnDepositMutation = useReturnDeposit({
    escrowId: sellerEscrowIds![0],
  });
  const takePaymentMutation = useTakePayment({ escrowId: sellerEscrowIds![0] });
  const withdrawCollateralMutation = useWithdrawCollateral({
    escrowId: sellerEscrowIds![0],
  });

  // TODO DRY for repeated code in CreateEscrow.tsx
  const [arbiter, setArbiter] = useState("");
  const [arbiterAsset, setArbiterAsset] = useState("");
  const [arbiterFee, setArbiterFee] = useState("");

  const proposeArbiterMutation = useProposeArbiter({
    arbiterAddress: arbiter,
    arbiterAsset,
    arbiterFee,
    escrowId: sellerEscrowIds[0],
    setArbiterAddress: setArbiter,
    setArbiterAsset,
    setArbiterFee,
  });

  const handleArbiterAddressChange = (event: ChangeEvent<HTMLInputElement>) => {
    const newArbiter = event.target.value;
    setArbiter(newArbiter);
  };

  const handleArbiterAssetChange = (event: ChangeEvent<HTMLInputElement>) => {
    const newAssetId = event.target.value;
    setArbiterAsset(newAssetId);
  };

  const handleArbiterFeeChange = (event: ChangeEvent<HTMLInputElement>) => {
    const newFee = event.target.value;
    setArbiterFee(newFee);
  };

  return (
    <Layout>
      <Flex direction="column" justify="center">
        <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
          <CreateEscrow />
          {showBalances && <ShowBalances />}
        </Flex>
        <Flex justify="center">
          <Card css={{ flex: "1", maxW: "900px", marginTop: "$5" }}>
            <Card.Header>Seller Escrows</Card.Header>
            {!!sellerEscrows && sellerEscrows.length > 0 ? (
              <>
                <EscrowInfo escrows={sellerEscrows} />

                {!!sellerEscrows[0].state.Pending && (
                  <Card.Footer justify="space-evenly">
                    <ArbiterInputContainer
                      onArbiterAddressChange={handleArbiterAddressChange}
                      onAssetIdChange={handleArbiterAssetChange}
                      onFeeChange={handleArbiterFeeChange}
                      arbiterAddress={arbiter}
                      asset={arbiterAsset}
                      feeAmount={arbiterFee}
                      addressLabel="Propose arbiter address input"
                      assetLabel="Propose arbiter asset input"
                      feeLabel="Propose arbiter fee input"
                    />
                    <Button
                      aria-label="Propose arbiter button"
                      onPress={() => proposeArbiterMutation.mutate()}
                    >
                      Propose Arbiter
                    </Button>
                  </Card.Footer>
                )}

                {!!sellerEscrows[0].state.Pending && (
                  <Card.Footer justify="space-evenly">
                    <Button
                      aria-label="Return deposit"
                      onPress={() => returnDepositMutation.mutate()}
                    >
                      Return Deposit
                    </Button>
                    <Button
                      aria-label="Take payment"
                      onPress={() => takePaymentMutation.mutate()}
                    >
                      Take Payment
                    </Button>
                    <Button
                      aria-label="Withdraw collateral"
                      onPress={() => withdrawCollateralMutation.mutate()}
                    >
                      Withdraw Collateral
                    </Button>
                  </Card.Footer>
                )}

                <Card.Footer direction="row-reverse" gap="$4">
                  <Button leftIcon="DotsThree">Show all escrows</Button>
                </Card.Footer>
              </>
            ) : (
              <>
                <Card.Body>Seller has no escrows</Card.Body>
              </>
            )}
          </Card>
        </Flex>
      </Flex>
    </Layout>
  );
}
