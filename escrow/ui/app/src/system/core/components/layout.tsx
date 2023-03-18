import { Flex, Stack } from "@fuel-ui/react";
import type { ReactNode } from "react";
import { Header } from "./header";

type LayoutProps = {
  children?: ReactNode;
};

export function Layout({ children }: LayoutProps) {
  return (
    <Stack
      css={{
        background:
          "linear-gradient(rgb(255 120 229) 30%, rgb(125 85 255) 130%)",
      }}
    >
      <Header />
      <Flex
        css={{
          height: "100vh",
          width: "100%",
          flexDirection: "column",
          marginTop: "5%",
        }}
      >
        {children}
      </Flex>
    </Stack>
  );
}
