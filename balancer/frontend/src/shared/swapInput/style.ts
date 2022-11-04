import styled, { keyframes } from "styled-components";

interface InputWrapperProps {
  margin?: string;
  switchSwap?: boolean;
  position?: "top" | "bottom";
  showModalList?: boolean;
}

const MoveTopAnimation = (switchSwap?: boolean) => keyframes`
  0%{
    transform: ${switchSwap ? `translateY(0px)` : `translateY(115px)`} 
  }

  50%{
    opacity: 0.5 ;
  }

  100%{
    transform: ${switchSwap ? `translateY(115px)` : `translateY(0px)`} 
  }
`;

const MoveBottomAnimation = (switchSwap?: boolean) => keyframes`
  0%{
    transform: ${switchSwap ? `translateY(0px)` : `translateY(-115px)`} 
  }

  50%{
    opacity: 0.5 ;
  }

  100%{
    transform: ${switchSwap ? `translateY(-115px)` : `translateY(0px)`} 
  }
`;

export const InputWrapper = styled.div<InputWrapperProps>`
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  margin: ${(props) => props.margin};
  animation: ${(props) =>
      props.switchSwap !== undefined &&
      (props.position === "top"
        ? MoveTopAnimation(props.switchSwap)
        : MoveBottomAnimation(props.switchSwap))}
    0.5s;
  animation-fill-mode: forwards;

  .maxButton {
    position: absolute;
    right: ${(props) => (props.showModalList ? `8.5rem` : `0.3rem`)};
  }
  /* Chrome, Safari, Edge, Opera */
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  /* Firefox */
  input[type="number"] {
    -moz-appearance: textfield;
  }
`;

interface BalanceWrapperProps {
  account?: boolean;
}

const MoveUpAnimation = ({ account }: BalanceWrapperProps) => keyframes`
  0%{
    transform: ${account ? "translateY(0px)" : "translateY(-40px)"} ;
    opacity: ${account ? 0 : 1};    
  };
  
  100%{
    transform: ${account ? `translateY(-40px)` : `translateY(0px)`};
    opacity: ${account ? 1 : 0};
  };
`;

export const BalanceWrapper = styled.div<BalanceWrapperProps>`
  position: absolute;
  right: 0px;
  white-space: nowrap;
  color: #fff;
  animation: ${(props: any) => MoveUpAnimation(props)} 0.5s ease-out;
  animation-fill-mode: forwards;
`;

export const SelectorButton = styled.div`
  cursor: pointer;
  display: flex;
  align-items: center;
  color: #fff;
  height: 3.5rem;
  padding: 0rem 1rem;
  border-radius: 0.3rem;
  white-space: nowrap;
  background-color: #334155;
  /* margin-bottom: 10px; */
`;

interface TickerContainerProps {
  onChange?: React.MouseEventHandler<HTMLButtonElement>;
}

export const TokenContainer = styled.div<TickerContainerProps>`
  display: flex;
  margin: 12px 16px;
  cursor: pointer;
  padding: 12px 16px;
  border-radius: 12px;
  background: rgba(47, 57, 74, 0.5);
  :hover {
    background: rgb(65, 76, 94);
  }
`;

export const DeleteButton = styled.div`
  height: 15px;
  line-height: 15px;
  width: 15px;
  font-weight: bold;
  border-radius: 50%;
  background-color: #fff;
  color: white;
  text-align: center;
  cursor: pointer;
  margin: 0 0 0 10px;
  display: flex;
  justify-content: center;
  opacity: 0.5;
  :hover {
    scale: 1.1;
    opacity: 1;
  }
  img {
    width: 15px;
    display: block;
  }
`;
