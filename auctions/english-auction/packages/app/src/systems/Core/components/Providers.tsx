import { ThemeProvider } from "@fuel-ui/react";
import type { ReactNode } from "react";
import { QueryClientProvider } from "react-query";

import { queryClient } from "../utils";

type AppProps = {
  children?: ReactNode;
};

export function Providers({ children }: AppProps) {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>{children}</ThemeProvider>
    </QueryClientProvider>
  );
}
