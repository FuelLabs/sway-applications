import { Box, Button, Container, Stack } from "@fuel-ui/react";
import { useLocation, useNavigate } from "react-router-dom";

import { Pages } from "../../../types";
import { useWallet } from "../hooks/useWallet";

import { WalletWidget } from "./WalletWidget";

export function TopNav() {
  const wallet = useWallet();
  const navigate = useNavigate();
  const location = useLocation();

  return (
    <Box css={{ borderBottom: "1px solid $gray5", background: "$gray3" }}>
      <Container
        css={{ py: "$8", display: "flex", justifyContent: "flex-end" }}
      >
        <Stack gap="$2" direction="row" css={{ marginRight: "20%" }}>
          <Button
            variant={location.pathname === Pages.seller ? undefined : "ghost"}
            leftIcon="Package"
            onPress={() => navigate(Pages.seller)}
            aria-label="Go to seller page"
          >
            Seller
          </Button>
          <Button
            variant={location.pathname === Pages.buyer ? undefined : "ghost"}
            leftIcon="Money"
            onPress={() => navigate(Pages.buyer)}
            aria-label="Go to buyer page"
          >
            Buyer
          </Button>
          <Button
            variant={location.pathname === Pages.arbiter ? undefined : "ghost"}
            leftIcon="Scales"
            onPress={() => navigate(Pages.arbiter)}
            aria-label="Go to arbiter page"
          >
            Arbiter
          </Button>
        </Stack>
        {/* TODO figure out if we want a theme switcher
           <IconButton
            variant="link"
            aria-label="Theme Switcher"
            icon={theme === "light" ? "MoonIcon" : "SunIcon"}
            color={theme === "light" ? "gray" : "yellow"}
            onPress={toggleTheme}
          /> */}

        {wallet && <WalletWidget />}
      </Container>
    </Box>
  );
}
