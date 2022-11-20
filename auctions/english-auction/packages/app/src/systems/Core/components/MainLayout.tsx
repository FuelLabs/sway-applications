import type { ReactNode } from "react";

import { useConnectWallet } from "../hooks/useConnectWallet";

import { Header } from "./Header";

type MainLayoutProps = {
  children?: ReactNode;
};

export function MainLayout({ children }: MainLayoutProps) {
  useConnectWallet();
  return (
    <>
      <Header />
      {children}
    </>
  );
}
