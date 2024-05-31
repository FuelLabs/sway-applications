import { useUploadFile } from "@/hooks/useUploadFile";
import { ChangeEvent, useEffect, useRef } from "react";
import type { Dispatch, SetStateAction } from "react";
import { Button } from "./Button";

type UploadButtonProps = {
    setCid: Dispatch<SetStateAction<string>>
}

export const UploadButton = ({ setCid }: UploadButtonProps) => {
  //const [file, setFile] = useState<File | null>(null);
  //const [cid, setCid] = useState("");

  const inputFile = useRef<HTMLInputElement>(null);

  const uploadFile = useUploadFile();

  useEffect(() => {
    if (uploadFile.data) {
      setCid(uploadFile.data);
    }
  }, [uploadFile.data]);

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length) {
      //setFile(e.target.files[0]);
      uploadFile.mutate(e.target.files[0]);
    }
  };

  return (
    <>
      <input
        type="file"
        id="file"
        ref={inputFile}
        onChange={handleChange}
        style={{ display: "none" }}
      />
      <Button onClick={() => inputFile.current?.click()}>
        {uploadFile.isPending ? "Uploading..." : "Upload"}
      </Button>
    </>
  );
};
