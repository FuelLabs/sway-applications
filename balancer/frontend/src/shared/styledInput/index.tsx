import { InputWrapper } from "./style";

interface InputProps {
  type?: string;
  disable?: boolean;
  states?: any;
  value?: any;
  onChange?: React.ChangeEventHandler<HTMLInputElement> | undefined;
  fullWidth?: boolean;
}

export const StyledInput = (props: InputProps) => {
  const { type, disable, states, onChange, value, fullWidth } = props;
  return (
    <InputWrapper
      type={type}
      onChange={onChange}
      disabled={disable}
      state={states}
      value={value}
      fullWidth={fullWidth}
      step="0.01"
      min="0"
      placeholder="0.0"
      // required
      autoComplete={"false"}
    />
  );
};
