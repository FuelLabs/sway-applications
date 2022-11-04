import { useEffect, useState } from "react";
import Card from "../../shared/card";
import SubmitButton from "../../shared/submitButton/SubmitButton";
import SwapInput from "../../shared/swapInput";
import { StyledAddressInput } from "../../styles/styles";
import { FormWrapper, Row, TokenListWrapper } from "./style";

interface I_Token {
  name: string;
  address: string;
  value?: string;
}
interface I_PoolProps {
  formValues: Array<I_Token>;
  setFormValues: (token: I_Token[]) => void;
  tokens: Array<I_Token>;
  poolId: String;
  setPoolId: (id: string) => void;
  registerPool: () => void;
}

const CreatePool = (props: I_PoolProps) => {
  const { formValues, setFormValues, tokens, poolId, setPoolId, registerPool } =
    props;
  const [selTokenList, setSelTokenList] = useState([
    formValues[0].name,
    formValues[1].name,
  ]);

  let handleValueChange = (e: any, i: number) => {
    let newFormValues = [...formValues];
    newFormValues[i].value = e.target.value;
    setFormValues(newFormValues);
  };
  let handleTokenChange = (e: string, i: number) => {
    let newFormValues = JSON.parse(JSON.stringify(formValues));
    newFormValues[i].name = e;
    // @ts-ignore
    newFormValues[i].address = tokens[e]?.address;
    setFormValues(newFormValues);

    // const selToken = [...selTokenList];
    // selToken[i] = e;
    // setselTokenList([...selToken]);
  };

  let addFormFields = () => {
    setFormValues([...formValues, { name: "", address: "", value: "" }]);
  };

  let removeFormFields = (i: number) => {
    let newFormValues = [...formValues];
    newFormValues.splice(i, 1);
    setFormValues(newFormValues);

    const selToken = [...selTokenList];
    selToken.splice(i, 1);
    setSelTokenList([...selToken]);
  };

  let handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    console.table(formValues);
    if (!poolId) {
      alert("Please enter pool id");
      return;
    }
    if (formValues?.length < 2) {
      alert("Please select atleast 2 tokens");
      return;
    }

    await registerPool();

    // let totalAllocation = 0;

    // formValues.forEach((token) => {
    //   if (token?.name) {
    //     if (token?.value) {
    //       totalAllocation += parseFloat(token.value);
    //     } else {
    //       alert("Please enter valid token weights");
    //       return;
    //     }
    //   } else {
    //     alert("Please select all token");
    //     return;
    //   }
    // });

    // if (totalAllocation === 100) {
    //   // next logic
    // } else {
    //   alert("Please make sure the total token weights add up to 100%");
    //   return;
    // }
  };

  useEffect(() => {
    const selToken = formValues.map((token) => (token.name ? token.name : ""));
    setSelTokenList([...selToken]);
  }, [formValues]);

  return (
    <FormWrapper onSubmit={(e) => handleSubmit(e)}>
      <Card
        title="Pool Contract ID"
        titleFontVariant="h6"
        backgroundColor="#162031"
        cardMaxWidth={"400"}
      >
        <StyledAddressInput
          type="text"
          placeholder="Enter Pool Contract ID"
          onChange={
            // setAmount(this)
            (evt: any) => {
              // console.log(evt.target.value);
              setPoolId(evt.target.value);
            }
          }
          required
        />
      </Card>
      <Card
        title="Choose tokens & weights"
        titleFontVariant="h6"
        backgroundColor="#162031"
        cardMaxWidth={"400"}
      >
        <TokenListWrapper>
          <Row>
            <h4>Token</h4>
            <h4>Weight</h4>
          </Row>
          {formValues.map((element: any, index: number) => (
            <div className="form-inline" key={index}>
              {/* <label>Name</label> */}
              {/* <input
            type="text"
            name="name"
            value={element.name || ""}
            onChange={(e) => handleChange(index, e)}
          /> */}

              <SwapInput
                position="top"
                onTokenChange={(val) => {
                  handleTokenChange(val, index);
                }}
                token={element.name}
                showModalList
                inputValue={element.value}
                onChangeInput={(e) => handleValueChange(e, index)}
                swapTokenList={Object.values(tokens).filter((token: any) => {
                  return !selTokenList.includes(token?.name);
                })}
                removeToken={() => removeFormFields(index)}
                formValues={formValues}
              />
              {/* <label>Email</label>
          <input
            type="text"
            name="email"
            value={element.email || ""}
            onChange={(e) => handleChange(index, e)}
          /> */}
              {/* {index ? (
              <button
                type="button"
                className="button remove"
                onClick={() => removeFormFields(index)}
              >
                Remove
              </button>
            ) : null} */}
            </div>
          ))}
        </TokenListWrapper>
        <div className="button-section">
          <button
            className="button-61"
            role="button"
            type="button"
            onClick={() => addFormFields()}
          >
            Add a token
          </button>
          <SubmitButton text="Create pool" />
        </div>
      </Card>
    </FormWrapper>
  );
};

export default CreatePool;
