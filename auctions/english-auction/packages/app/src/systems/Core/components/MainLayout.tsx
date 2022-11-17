import type { ReactNode } from "react";

import { useWallet } from "../hooks/useWallet";

import { Header } from "./Header";

type MainLayoutProps = {
  children?: ReactNode;
};

export function MainLayout({ children }: MainLayoutProps) {
  useWallet();
  return (
    <>
      <Header />
      {children}
    </>
  );
}
