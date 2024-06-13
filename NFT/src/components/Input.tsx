import { Input as BaseInput } from "@mui/material";
import clsx from "clsx";

export const Input: React.FC<{
  value?: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
  className?: string;
}> = ({ value, onChange, placeholder, className }) => {
  return (
    <BaseInput
      value={value}
      className={clsx([
        "mt-1",
        "shrink",
        "basis-2/3",
        "rounded-lg",
        "border-2",
        "border-solid",
        "border-zinc-500/25",
        "p-1",
        "font-sans",
        "outline-none",
        "md:-ml-2",
        "md:mt-2",
        "md:p-2",
        "bg-transparent",
        "placeholder:text-zinc-400",
        "text-zinc-50",
        className,
      ])}
      onChange={onChange}
      placeholder={placeholder}
    />
  );
};
