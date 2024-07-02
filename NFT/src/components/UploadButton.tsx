import { ChangeEvent, useRef } from "react";
import type { Dispatch, SetStateAction } from "react";
import { Button } from "./Button";

type UploadButtonProps = {
  setFile: Dispatch<SetStateAction<File | undefined>>;
};

export const UploadButton = ({ setFile }: UploadButtonProps) => {
  const inputFile = useRef<HTMLInputElement>(null);

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length) {
      setFile(e.target.files[0]);
    }
  };

  return (
    <>
      <input
        type="file"
        id="file"
        accept=".jpg,.jpeg,.png,.gif"
        ref={inputFile}
        onChange={handleChange}
        style={{ display: "none" }}
      />
      <Button onClick={() => inputFile.current?.click()}>
        Choose File
      </Button>
    </>
  );
};
