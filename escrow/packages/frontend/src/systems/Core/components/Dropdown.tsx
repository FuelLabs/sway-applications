import { cx, styled } from "@fuel-ui/css";
//import { createComponent } from "@fuel-ui/react/src/utils";
import { createElement } from "react";
import type { ChangeEvent, ReactNode } from "react";

interface Props {
  onChange: (event: ChangeEvent) => void;
  children: ReactNode;
  className: any;
}

export const Dropdown = styled("select", {});

// const Root = styled("select");
// export const Dropdown = createComponent<Props>(
//   ({ className, children, onChange, ...props }) => {
//     const classes = cx("dropdown", className);
//     return createElement(
//       Root,
//       { ...props, className: classes, onChange },
//       children
//     );
//   }
// );
