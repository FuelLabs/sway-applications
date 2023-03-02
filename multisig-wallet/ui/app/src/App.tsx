import { Stack, ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./system/core/utils/queryClient";
import { Header, PleaseConnect } from "./system/core/components";
import { useEffect, useState } from "react"

function App() {
  const [page, setPage] = useState(<PleaseConnect />)

  useEffect(() => {
    async function main() {
        const isConnected = await window.fuel.isConnected();
        if (!isConnected) {
            setPage(<PleaseConnect />);
        } else {
            setPage(<>{page}</>);
        }
    }
    main();
  }, [page]);

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <Header setPage={setPage} />
        <Stack align="center" css={{ background: 'rgb(209 226 237)', height: "100vh" }}>{page}</Stack>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
