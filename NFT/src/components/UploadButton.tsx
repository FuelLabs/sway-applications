import { useUploadFile } from "@/hooks/useUploadFile";
import { ChangeEvent, useEffect, useRef, useState } from "react";
import { Button } from "./Button";

export const UploadButton = () => {
  const [file, setFile] = useState<File | null>(null);
  const [cid, setCid] = useState("");

  console.log(`file`, file);

  console.log(`cid`, cid);

  const inputFile = useRef<HTMLInputElement>(null);

  const uploadFile = useUploadFile();

  useEffect(() => {
    if (uploadFile.data) {
      setCid(uploadFile.data);
    }
  }, [uploadFile.data]);

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length) {
      setFile(e.target.files[0]);
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
