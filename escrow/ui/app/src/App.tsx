import { ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./system/core/utils/queryClient";
import { Layout } from "./system/core/components";
import { BrowserRouter } from "react-router-dom";
import { AppRoutes } from "./system/core/components/routes";

function App() {
  return (
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider>
          <Layout>
            <AppRoutes />
          </Layout>
        </ThemeProvider>
      </QueryClientProvider>
    </BrowserRouter>
  );
}

export default App;
