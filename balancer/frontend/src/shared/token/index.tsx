import { Spacer, Text } from "../shared";
import { TokenWrapper } from "./style";

interface TokenProps {
  token?: string;
  label?: string;
  labelColor?: string;
  size?: string;
}

const Token = (props: TokenProps) => {
  const { token = "unknown", label, labelColor, size } = props;

  return (
    <TokenWrapper labelColor={labelColor} size={size}>
      <img
        src={require(`../../assets/tokens/${token?.toLowerCase()}.png`)}
        alt={token}
      />
      {label ? (
        <Spacer marginLeft="0.5rem">
          <Text variants="normal" color={props.labelColor}>
            {label}
          </Text>
        </Spacer>
      ) : null}
    </TokenWrapper>
  );
};

export default Token;
