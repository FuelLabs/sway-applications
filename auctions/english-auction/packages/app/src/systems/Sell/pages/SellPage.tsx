import { CreateAuction } from "../components/CreateAuction";

import { MainLayout } from "~/systems/Core/components/MainLayout";

export function SellPage() {
  return (
    <MainLayout>
      <CreateAuction />
    </MainLayout>
  );
}
