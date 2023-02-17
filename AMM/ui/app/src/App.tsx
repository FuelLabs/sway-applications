import { ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./utils/queryClient";
import { InfoPage } from "./pages";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <InfoPage />
      </ThemeProvider>
    </QueryClientProvider >
  );
}

export default App;
