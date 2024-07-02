import { Button as BaseButton, ButtonProps } from "@mui/material";
import clsx from "clsx";

export const Button = ({
  children,
  className,
  variant = "contained",
  ...props
}: ButtonProps) => {
  return (
    <BaseButton
      {...props}
      variant={variant}
      className={clsx("btn", "btn-primary", className)}
    >
      {children}
    </BaseButton>
  );
};
