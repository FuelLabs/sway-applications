// @ts-nocheck
import { BoxCentered, Heading, ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./utils/queryClient";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <BoxCentered>
          <Heading as="h1">
            TBA
          </Heading>
        </BoxCentered>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
