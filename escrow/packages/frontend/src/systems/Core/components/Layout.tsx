import { cx, globalCss } from "@fuel-ui/css";
import { Box, Spinner } from "@fuel-ui/react";
import type { FC, ReactNode } from "react";

import { TopNav } from "./TopNav";

interface Props {
  title?: string;
  isLoading?: boolean;
  className?: string;
  isBlank?: boolean;
  children: ReactNode;
}

export const Layout: FC<Props> = ({
  children,
  isLoading,
  className,
  isBlank,
}) => {
  globalStyles();
  return (
    <>
      {isLoading ? (
        <Box as="main" className={cx("loading", className)}>
          <Spinner />
        </Box>
      ) : (
        <Box as="main" className={className}>
          {!isBlank && <TopNav />}
          {children}
        </Box>
      )}
    </>
  );
};

const globalStyles = globalCss({
  "body, #root, #root > main": {
    minH: "$screenH",
  },
  "#root > main:not(.loading)": {
    display: "flex",
    flexDirection: "column",
  },
  "#root > main.loading": {
    is: ["centered"],
  },
});
