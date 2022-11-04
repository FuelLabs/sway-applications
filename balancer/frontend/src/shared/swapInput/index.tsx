import { useState } from "react";
import CustomModal from "../modal/modal";
import { StyledInput } from "../styledInput";
import Token from "../token";
import {
  DeleteButton,
  InputWrapper,
  SelectorButton,
  TokenContainer,
} from "./style";

interface SwapInputProps {
  switchSwap?: boolean;
  inputValue?: string | number;
  onChangeInput?: React.ChangeEventHandler<HTMLInputElement>;
  onMaxButtonClick?: React.MouseEventHandler<HTMLButtonElement>;
  onTokenChange?: (token: string) => void;
  token?: string;
  balance?: string;
  swapTokenList?: Array<{
    name: string;
    address: string;
  }>;
  position?: "top" | "bottom";
  showModalList?: boolean;
  removeToken?: () => void;
  formValues: any;
}

const SwapInput = (props: SwapInputProps) => {
  const {
    switchSwap,
    inputValue,
    onChangeInput,
    onMaxButtonClick,
    onTokenChange,
    token,
    balance,
    swapTokenList,
    position,
    showModalList,
    removeToken,
    formValues,
  } = props;

  const [openModal, setOpenModal] = useState(false);

  return (
    <InputWrapper
      position={position}
      switchSwap={switchSwap}
      showModalList={showModalList}
      margin="10px 0"
    >
      {/* <div className="maxButton">
        <Button onClick={onMaxButtonClick}>Max</Button>
      </div> */}

      {showModalList && (
        <>
          {token ? (
            <SelectorButton onClick={() => setOpenModal(true)}>
              <Token
                size="40px"
                token={token}
                label={token}
                labelColor={"#fff"}
              />
              &nbsp; &#9660;
            </SelectorButton>
          ) : (
            <SelectorButton onClick={() => setOpenModal(true)}>
              Select a token &nbsp; &#9660;
            </SelectorButton>
          )}
          <CustomModal
            show={openModal}
            toggleModal={() => {
              setOpenModal(!openModal);
            }}
            heading="Select a token"
          >
            {swapTokenList?.map((val, i) => (
              <TokenContainer
                key={i}
                onClick={() => {
                  onTokenChange?.(val.name);
                  setOpenModal(false);
                }}
              >
                <Token token={val.name} label={val.name} labelColor={"#ccc"} />
              </TokenContainer>
            ))}
          </CustomModal>
        </>
      )}
      <StyledInput
        fullWidth
        value={inputValue}
        onChange={onChangeInput}
        type="number"
      />
      <span style={{ fontWeight: "bold" }}>%</span>
      <span>
        {formValues?.length > 2 && (
          <DeleteButton onClick={removeToken}>
            <img
              src={require(`../../assets/icons/delete-icon.png`)}
              alt={token}
            />
          </DeleteButton>
        )}
      </span>
    </InputWrapper>
  );
};

export default SwapInput;
