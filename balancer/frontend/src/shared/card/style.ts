import styled from "styled-components";

interface CardWrapperProps {
  cardMaxWidth?: number | string;
  backgroundColor?: string;
}

export const CardWrapper = styled.div<CardWrapperProps>`
  display: flex;
  flex-direction: column;
  align-self: center;
  width: 100%;
  max-width: ${(props) => props.cardMaxWidth}px;
  margin: 0 auto;
  min-height: auto;
  padding: 2rem 3rem;
  background: ${(props) => props.backgroundColor || `#151b24`};
  transition: all 300ms ease-in-out;
  border-radius: 12px;
`;
