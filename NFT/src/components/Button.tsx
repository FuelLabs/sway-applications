import { Button as BaseButton } from "@mui/material";
import clsx from "clsx";

export const Button: React.FC<{
  children: React.ReactNode;
  disabled?: boolean;
  onClick?: () => void;
  className?: string;
}> = ({ disabled, children, onClick, className }) => {
  return (
    <BaseButton
      variant="contained"
      disabled={disabled}
      className={clsx("btn", "btn-primary", className)}
      onClick={onClick}
    >
      {children}
    </BaseButton>
  );
};
