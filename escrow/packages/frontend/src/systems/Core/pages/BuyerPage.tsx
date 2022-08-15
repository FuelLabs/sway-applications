import { useAtomValue } from "jotai";
import { Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";

export default function BuyerPage() {
  const showBalances = useAtomValue(showBalancesAtom);

  return (
    <Layout>
      <Flex css={{ flexDirection: "row-reverse", justifyContent: "center" }}>
        {showBalances && <ShowBalances />}
        HELLO BUYER
      </Flex>
    </Layout>
  );
}
