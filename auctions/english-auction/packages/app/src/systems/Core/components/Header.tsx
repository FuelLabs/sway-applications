import { cx } from "@fuel-ui/css";
import { Button, ButtonGroup, Image, Grid } from "@fuel-ui/react";
import type { ButtonProps } from "@fuel-ui/react";
import type { ComponentType, ReactNode } from "react";
import { BiDollarCircle } from "react-icons/bi";
import { MdSwapCalls } from "react-icons/md";
import { useLocation, useNavigate } from "react-router-dom";

import { useBreakpoint } from "../hooks/useBreakpoint";
import { relativeUrl } from "../utils";

import { Pages } from "~/types";

type HeaderNavProps = ButtonProps & {
  onPress: () => void;
  isActive: boolean;
  icon: ComponentType<any>; // eslint-disable-line @typescript-eslint/no-explicit-any
  children: ReactNode;
};

const HeaderNav = ({
  onPress,
  isActive,
  icon: Icon,
  children,
  ...props
}: HeaderNavProps) => {
  const breakpoint = useBreakpoint();
  return (
    <Button
      {...props}
      variant="ghost"
      size="lg"
      onPress={onPress}
      className={cx("header--navItem", {
        "header--navItemActive": isActive,
      })}
      css={{ isFull: `${breakpoint === "sm"}` }}
    >
      <Icon
        className={cx("text-primary-gray", { "text-primary-400": isActive })}
      />
      {children}
    </Button>
  );
};

export const Header = () => {
  const navigate = useNavigate();
  const location = useLocation();

  return (
    <Grid templateColumns="repeat(2, 1fr)" css={{ padding: "$4" }}>
      <Grid.Item area="header">
        <Image
          onClick={() => navigate("/")}
          src={relativeUrl("/fuel-logo-512x512.png")}
          alt="english-auction"
          className="cursor-pointer"
          height="40px"
          width="40px"
        />
      </Grid.Item>
      <Grid.Item area="header">
        <ButtonGroup>
          <HeaderNav
            icon={MdSwapCalls}
            onPress={() => navigate(Pages.sell)}
            isActive={location.pathname === Pages.sell}
          >
            Sell
          </HeaderNav>
          <HeaderNav
            icon={BiDollarCircle}
            onPress={() => navigate(Pages.buy)}
            isActive={location.pathname.includes(Pages.buy)}
          >
            Buy
          </HeaderNav>
        </ButtonGroup>
      </Grid.Item>
    </Grid>
  );
};
