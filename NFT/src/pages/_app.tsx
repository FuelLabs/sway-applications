import "@/styles/globals.css";
import type { NextPage } from "next";
import type { AppProps } from "next/app";
import React from "react";
import Head from "next/head";
import { AppProvider } from "@/components/Provider";
import { useRouter } from "next/router";
import { getNFTLayout } from "@/utils/getNFTLayout";

export type NextPageWithLayout = NextPage & {
  getLayout?: (page: React.ReactElement) => React.ReactNode;
};

type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

export default function App({ Component, pageProps }: AppPropsWithLayout) {
  const router = useRouter();
  const getLayout = router.route.includes("nft")
    ? getNFTLayout
    : (page: React.ReactElement) => {
        return (
              <>{page}</>
        );
      };

  return (
    <React.StrictMode>
      <AppProvider>
        <Head>
          <title>Fuel App</title>
          <link rel="icon" href="/fuel.ico" />
        </Head>

        {getLayout(<Component {...pageProps} />)}
      </AppProvider>
    </React.StrictMode>
  );
}
