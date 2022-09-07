import { useAtomValue } from "jotai";
import { Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";
import { Card } from "@fuel-ui/react";
import { useArbiterEscrows } from "../hooks/useArbiterEscrows";
import { EscrowInfo } from "../components/EscrowInfo";

export default function BuyerPage() {
  const showBalances = useAtomValue(showBalancesAtom);
  const arbiterEscrows = useArbiterEscrows();

  return (
    <Layout>
      <Flex css={{ flexDirection: "row", justifyContent: "center" }}>
        <EscrowInfo escrows={arbiterEscrows} />
        {showBalances && <ShowBalances />}
      </Flex>
    </Layout>
  );
}
