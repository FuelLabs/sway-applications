import type { LinkProps } from "@fuel-ui/react";
import { Link as Root } from "@fuel-ui/react";
import type { LinkProps as RouterLinkProps } from "react-router-dom";
import { NavLink as RouterLink } from "react-router-dom";

export function Link(props: RouterLinkProps & LinkProps) {
  return <Root as={RouterLink} {...props} />;
}
