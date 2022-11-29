import type { ReactNode } from "react";

import { Header } from "./Header";

type MainLayoutProps = {
  children?: ReactNode;
};

export function MainLayout({ children }: MainLayoutProps) {
  return (
    <>
      <Header />
      {children}
    </>
  );
}
