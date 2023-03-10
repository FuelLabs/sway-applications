import { Stack } from "@fuel-ui/react";
import type { ReactNode } from "react";
import { Header } from "./header";

type LayoutProps = {
  children?: ReactNode;
};

export function Layout({ children }: LayoutProps) {
  return (
    <>
      <Header />
      <Stack align="center" css={{ background: 'rgb(209 226 237)', height: "100vh" }}>{children}</Stack>
    </>
  );
}
