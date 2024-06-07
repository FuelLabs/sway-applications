import formidable, { File } from "formidable";
import fs from "fs";
import pinataSDK, { PinataPinOptions } from "@pinata/sdk";
import { getRandomB256 } from "fuels";
import { NextApiRequest, NextApiResponse } from "next";

const pinata = new pinataSDK({ pinataJWTKey: process.env.PINATA_JWT });

export const config = {
  api: {
    bodyParser: false,
  },
};

const saveFile = async (file: File, nftName: string, nftDescription: string) => {
  try {
    const stream = fs.createReadStream(file.filepath);
    const fileCid = getRandomB256();
    const options: PinataPinOptions = {
      pinataMetadata: {
        name: fileCid,
        keyvalues: {
          nftName,
          nftDescription,
        }
      },
      pinataOptions: {
        wrapWithDirectory: true
      },
    };
    const response = await pinata.pinFileToIPFS(stream, options);
    fs.unlinkSync(file.filepath);

    return response;
  } catch (error) {
    throw error;
  }
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  try {
    if (req.method === "POST") {
      const form = formidable({ keepExtensions: true });
      form.parse(req, async (err, fields, files) => {
        if (err || !files.file || files.file.length === 0) {
          console.error({ err });
          return res.status(500).send("Upload Error");
        }
        const response = await saveFile(files.file[0], fields.nftName?.at(0) || "", fields.nftDescription?.at(0) || "");
        const { IpfsHash } = response;

        return res.send(IpfsHash);
      });
    } else if (req.method === "GET") {
        const hashContains = JSON.stringify(req.query) === JSON.stringify({}) ? undefined : req.query['cid'] as string;
        // TODO: support pagination for an explore page
        const nftData = await pinata.pinList({ hashContains, status: "pinned" });
        res.json(nftData.rows);
    }
  } catch (error) {
    console.error(error);
    res.status(500).send("Server Error");
  }
}
