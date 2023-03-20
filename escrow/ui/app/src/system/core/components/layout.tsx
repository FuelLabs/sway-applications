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
          // "linear-gradient(197.05deg, rgb(14, 34, 27) 0%, rgb(7, 22, 20) 22.2%, rgb(12, 14, 13) 40.7%)"
          "linear-gradient(190deg, rgb(30, 113, 84) 10%, rgb(33 91 73) 63%)",
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
