import { Home } from "../components/Home";

import { MainLayout } from "~/systems/Core/components/MainLayout";

export function HomePage() {
  return (
    <div className="homePage">
      <MainLayout>
        <Home />
      </MainLayout>
    </div>
  );
}
