import { ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./system/core/utils/queryClient";
import { PleaseConnect } from "./system/core/components";
import { useEffect, useState } from "react"
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Layout } from "./system/core/components";
import { CreatePage } from "./system/create/pages";
import { ExecutePage } from "./system/execute/pages";
import { UpdatePage } from "./system/update/pages";
import { ViewPage } from "./system/view/pages";

function App() {
  // const [page, setPage] = useState(<PleaseConnect />)

  // useEffect(() => {
  //   async function main() {
  //       const isConnected = await window.fuel.isConnected();
  //       if (!isConnected) {
  //           setPage(<PleaseConnect />);
  //       } else {
  //           setPage(<></>);
  //       }
  //   }
  //   main();
  // }, [page]);

  return (
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider>
          <Layout>
            <Routes>
              <Route path="/create" element={<CreatePage />} />
              <Route path="/execute" element={<ExecutePage />} />
              <Route path="/update" element={<UpdatePage />} />
              <Route path="/view" element={<ViewPage />} />
            </Routes>
          </Layout>
        </ThemeProvider>
      </QueryClientProvider>
    </BrowserRouter>
  );
}

export default App;
