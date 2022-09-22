import { useAtomValue } from "jotai";
import { useState } from "react";
import toast from "react-hot-toast";
import type { ChangeEvent } from "react";
import { useQueryClient } from "react-query";
import { Button, Card, Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom, walletIndexAtom } from "../jotai";
import { useSellerEscrows } from "../hooks/useSellerEscrows";
import { formatValue } from "../utils/helpers";
import { DECIMAL_PLACES } from "@/config";
import { useContract } from "../hooks/useContract";
import { ArbiterInputContainer } from "../components/ArbiterInputContainer";
import { parseInputValueBigInt } from "../utils/math";
import { ArbiterInput } from "@/types/contracts/EscrowAbi";
import { useReturnDeposit } from "../hooks/useReturnDeposit";
import { EscrowInfo } from "../components/EscrowInfo";

export default function SellerPage() {
  const queryClient = useQueryClient();
  const showBalances = useAtomValue(showBalancesAtom);
  const walletIdx = useAtomValue(walletIndexAtom);
  const sellerEscrows = useSellerEscrows();
  const contract = useContract();
  const returnDepositMutation = useReturnDeposit({ escrowId: BigInt(0) });

  // TODO DRY for repeated code in CreateEscrow.tsx
  const [arbiter, setArbiter] = useState("");
  const [arbiterAsset, setArbiterAsset] = useState("");
  const [arbiterFee, setArbiterFee] = useState<string | undefined>();

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

  const handleProposeArbiter = (escrowId: bigint) => {
    // TODO make this more flexible for assets of arbitrary decimal precision
    const actualFee = parseInputValueBigInt(arbiterFee!);
    // TODO figure out how to get this to work with contract id too
    let arbiterArg: ArbiterInput = {
      address: { Address: { value: arbiter } },
      asset: { value: arbiterAsset },
      fee_amount: actualFee,
    };
    // TODO change this from multiCall to single call once https://github.com/FuelLabs/fuels-ts/issues/445
    // is fixed
    // TODO don't hardcode gas and byte prices
    const result = contract!
      .multiCall([
        contract!.functions.propose_arbiter(arbiterArg, escrowId).callParams({
          forward: [actualFee, arbiterAsset]
        }),
      ])
      .txParams({
        gasPrice: BigInt(5),
        bytePrice: BigInt(5),
        gasLimit: 100_000_000
      }).call();
    console.log("result: ", result);
    toast.promise(result, {
      loading: 'Transaction loading...',
      success: 'Escrow created successfully',
      error: `Transaction reverted!`
    });
    setArbiter("");
    setArbiterAsset("");
    setArbiterFee("");
    // Trigger query to update show balances component
    queryClient.fetchQuery(['EscrowPage-balances', walletIdx]);
    // Trigger query to update seller escrows
    queryClient.fetchQuery(['SellerPage-sellerEscrowIds', contract]);
    queryClient.fetchQuery(["SellerEscrows", contract]);
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
                    <Button onPress={() => handleProposeArbiter(BigInt(0))}>
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
