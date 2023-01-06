/* eslint-disable @typescript-eslint/no-explicit-any */
import { Button, Input } from "@fuel-ui/react";
import { waitFor, fireEvent, render, screen, act } from "@fuel-ui/test-utils";
import { yupResolver } from "@hookform/resolvers/yup";
import { useForm } from "react-hook-form";
import * as yup from "yup";

import type { ControlledFieldProps } from "./ControlledField";
import { ControlledField } from "./ControlledField";

const onSubmitHandler = jest.fn();
const schema = yup
  .object({ title: yup.string().min(2).required("Title is required") })
  .required();

const Content = (props: Partial<ControlledFieldProps>) => {
  const { control, handleSubmit, watch } = useForm({
    resolver: yupResolver(schema),
    reValidateMode: "onChange",
    mode: "onChange",
    defaultValues: {
      title: "",
    },
  });

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <span data-testid="value">{watch("title")}</span>
      <ControlledField
        {...props}
        name="title"
        label="Title"
        control={control}
        render={({ field }) => (
          <Input>
            <Input.Field {...field} placeholder="Type your title" />
          </Input>
        )}
      />
      <Button type="submit">Send</Button>
    </form>
  );
};

describe("ControlledField", () => {
  it("should render a valid input using react-hook-form", async () => {
    const { user } = render(<Content />);
    const field = screen.getByPlaceholderText("Type your title");
    const value = screen.getByTestId("value");
    const btn = screen.getByText("Send");

    expect(field).toBeInTheDocument();
    expect(value.innerText).toBeFalsy();

    fireEvent.input(field, { target: { value: "Fuel" } });
    expect(await screen.findByText("Fuel")).toBeInTheDocument();

    await act(async () => {
      await user.click(btn);
    });
    await waitFor(async () => {
      expect(onSubmitHandler).toBeCalledTimes(1);
    });
  });

  it("should render field error message", async () => {
    render(<Content />);
    const field = screen.getByPlaceholderText("Type your title");

    fireEvent.input(field, { target: { value: "F" } });
    expect(await screen.findByText(/at least/)).toBeInTheDocument();
  });

  it("should control props work correctly", async () => {
    render(<Content isRequired isDisabled />);
    const field = screen.getByPlaceholderText("Type your title");
    const label = screen.getByText("Title");
    const classList = Array.from(label.classList);

    expect(label).toBeInTheDocument();
    expect(classList.some((c: any) => c.includes("required"))).toBe(true);
    expect(field).toBeDisabled();
  });
});
