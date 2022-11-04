import styled from "styled-components";

export const SubmitButtonContainer = styled.div`
  width: 100%;
  display: flex;
  justify-content: center;
  margin-top: 50px;
  .button-64 {
    width: 100%;
    align-items: center;
    background-image: linear-gradient(144deg, #af40ff, #5b42f3 50%, #00ddeb);
    border: 0;
    border-radius: 8px;
    box-shadow: rgba(151, 65, 252, 0.2) 0 15px 30px -5px;
    box-sizing: border-box;
    color: #ffffff;
    display: flex;
    font-size: 20px;
    justify-content: center;
    line-height: 1em;
    max-width: 100%;
    min-width: 140px;
    padding: 3px;
    text-decoration: none;
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
    white-space: nowrap;
    cursor: pointer;
  }

  .button-64:active,
  .button-64:hover {
    outline: 0;
  }

  .button-64 span {
    background-color: rgb(5, 6, 45);
    font-weight: bold;
    padding: 16px 24px;
    border-radius: 6px;
    width: 100%;
    /* height: 100%; */
    transition: 300ms;
  }

  .button-64:hover span {
    background: none;
  }

  @media (min-width: 768px) {
    .button-64 {
      font-size: 24px;
      min-width: 196px;
    }
  }
`;
