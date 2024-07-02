import { Input as BaseInput } from "@mui/material";
import clsx from "clsx";

export const inputStyle = [
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
];

export const Input: React.FC<{
  value?: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
  className?: string;
  type?: string;
}> = ({ value, onChange, placeholder, className, type }) => {
  return (
    <BaseInput
      value={value}
      className={clsx([...inputStyle, className])}
      onChange={onChange}
      placeholder={placeholder}
      type={type}
      inputProps={{ className: "placeholder:text-zinc-400 text-zinc-50" }}
    />
  );
};
