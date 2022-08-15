import { useAtomValue } from "jotai";
import { Flex } from "@fuel-ui/react";
import { CreateEscrow } from "../components/CreateEscrow";

import { Layout } from "../components/Layout";
import { ShowBalances } from "../components/ShowBalances";
import { showBalancesAtom } from "../jotai";

export default function SellerPage() {
  const showBalances = useAtomValue(showBalancesAtom);

  return (
    <Layout>
      <Flex css={{ flexDirection: "row-reverse", justifyContent: "center" }}>
        {showBalances && <ShowBalances />}
        <CreateEscrow />
      </Flex>
    </Layout>
  );
}
