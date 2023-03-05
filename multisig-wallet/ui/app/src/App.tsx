import { ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./system/core/utils/queryClient";
import { BrowserRouter } from "react-router-dom";
import { Layout } from "./system/core/components";
import { AppRoutes } from "./system/core/components/routes";
import { useEffect, useState } from "react";
import { PleaseConnect } from "./system/core/components";

function App() {
  const [connected, setConnection] = useState(true)

  // useEffect(() => {
  //   async function main() {
  //     const isConnected = await window.fuel.isConnected();
  //     if (!isConnected) {
  //       setConnection(false);
  //     } else {
  //       setConnection(true);
  //     }
  //   }
  //   main();
  // }, [connected]);

  return (
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider>
          <Layout>
            {connected ? <AppRoutes /> : <PleaseConnect />}
            {/* <AppRoutes /> */}
          </Layout>
        </ThemeProvider>
      </QueryClientProvider>
    </BrowserRouter>
  );
}

export default App;
