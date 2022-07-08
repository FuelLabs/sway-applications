import { cx, styled } from "@fuels-ui/css";
import { createComponent } from "@fuels-ui/react/src/utils";
import { createElement } from "react";
import type { ChangeEvent, ReactNode } from "react";

interface Props {
  onChange: (event: ChangeEvent) => void;
  children: ReactNode;
  className: any;
}

const Root = styled("select");
export const Dropdown = createComponent<Props>(
  ({ className, children, onChange, ...props }) => {
    const classes = cx("dropdown", className);
    return createElement(
      Root,
      { ...props, className: classes, onChange },
      children
    );
  }
);
