import { ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./system/core/utils/queryClient";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Layout } from "./system/core/components";
import { ConstructorPage } from "./system/constructor/pages";
import { ExecuteTransactionPage } from "./system/execute_transaction/pages";
import { TransferPage } from "./system/transfer/pages";
import { ThresholdPage } from "./system/threshold/pages";
import { WeightPage } from "./system/weight/pages";
import { ViewPage } from "./system/view/pages";

function App() {
  return (
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider>
          <Layout>
            <Routes>
              <Route path="/create" element={<ConstructorPage />} />
              <Route path="/execute" element={<ExecuteTransactionPage />} />
              <Route path="/transfer" element={<TransferPage />} />
              <Route path="/update-threshold" element={<ThresholdPage />} />
              <Route path="/update-weight" element={<WeightPage />} />
              <Route path="/view" element={<ViewPage />} />
            </Routes>
          </Layout>
        </ThemeProvider>
      </QueryClientProvider>
    </BrowserRouter>
  );
}

export default App;
