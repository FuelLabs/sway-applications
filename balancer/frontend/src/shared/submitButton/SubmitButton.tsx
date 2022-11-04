import { SubmitButtonContainer } from "./style";

function SubmitButton(props: any) {
  const { text, handleClick = undefined } = props;
  return (
    <SubmitButtonContainer>
      <button
        className="button-64"
        type={handleClick ? "button" : "submit"}
        onClick={handleClick}
      >
        <span className="text">{text}</span>
      </button>
    </SubmitButtonContainer>
  );
}

export default SubmitButton;
