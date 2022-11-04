import styled from "styled-components";

export const FormWrapper = styled.form`
  /* background-color: #1e293b; */
  padding: 30px;
  border-radius: 8px;
  color: #fff;
  display: flex;
  flex-direction: column;
  gap: 20px;
  /* CSS */
  .button-61 {
    background: transparent;
    border: 2px solid #6200ee;
    color: #fff;
    align-items: center;
    appearance: none;
    border-radius: 4px;
    box-shadow: rgba(0, 0, 0, 0.2) 0 3px 1px -2px,
      rgba(0, 0, 0, 0.14) 0 2px 2px 0, rgba(0, 0, 0, 0.12) 0 1px 5px 0;
    box-sizing: border-box;
    color: #fff;
    cursor: pointer;
    display: inline-flex;
    font-family: Roboto, sans-serif;
    font-size: 0.875rem;
    font-weight: 500;
    height: 36px;
    justify-content: center;
    letter-spacing: 0.0892857em;
    line-height: normal;
    min-width: 64px;
    outline: none;
    overflow: visible;
    padding: 0 16px;
    position: relative;
    text-align: center;
    text-decoration: none;
    text-transform: uppercase;
    transition: box-shadow 280ms cubic-bezier(0.4, 0, 0.2, 1);
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
    vertical-align: middle;
    will-change: transform, opacity;
  }

  .button-61:hover {
    box-shadow: rgba(0, 0, 0, 0.2) 0 2px 4px -1px,
      rgba(0, 0, 0, 0.14) 0 4px 5px 0, rgba(0, 0, 0, 0.12) 0 1px 10px 0;
  }

  .button-61:focus {
    box-shadow: rgba(0, 0, 0, 0.2) 0 2px 4px -1px,
      rgba(0, 0, 0, 0.14) 0 4px 5px 0, rgba(0, 0, 0, 0.12) 0 1px 10px 0;
  }

  .button-61:active {
    box-shadow: rgba(0, 0, 0, 0.2) 0 5px 5px -3px,
      rgba(0, 0, 0, 0.14) 0 8px 10px 1px, rgba(0, 0, 0, 0.12) 0 3px 14px 2px;
    background: #a46bf5;
  }
`;

export const Row = styled.div`
  display: flex;
  justify-content: space-between;
  border-bottom: 2px solid #ffffff94;
  /* margin-bottom: 10px; */
  padding: 0 10px;
  line-height: 0;
  p {
    font-size: 1.1rem;
    font-weight: bold;
  }
`;

export const TokenListWrapper = styled.div`
  background-color: #1e293b;
  padding: 10px 20px;
  border-radius: 8px;
  margin: 0 0 20px;
`;
