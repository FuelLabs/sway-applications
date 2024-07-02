import { Typography, TypographyProps } from "@mui/material";

type TextProps = TypographyProps;

export const Text = ({ children, className, ...props }: TextProps) => {
  return (
    <Typography {...props} className={`text-white font-sans ${className}`}>
      {children}
    </Typography>
  );
};
