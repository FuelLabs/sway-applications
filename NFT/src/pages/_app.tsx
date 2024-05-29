import { Layout } from "@/components/Layout";
import "@/styles/globals.css";
import type { AppProps } from "next/app";
import React from "react";
import { AppProvider } from "@/components/Provider";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <React.StrictMode>
      <AppProvider>
        <Layout>
          <Component {...pageProps} />
        </Layout>
      </AppProvider>
    </React.StrictMode>
  );
}
