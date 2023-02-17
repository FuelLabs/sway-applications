import { Stack, ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./system/core/utils/queryClient";
import { Layout } from "./system/core/components";
import { CreatePage } from "./system/create/pages";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <Layout>
          <Stack>
            {/* <CreatePage /> */}
          </Stack>
        </Layout>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
