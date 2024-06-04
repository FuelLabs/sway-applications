import formidable, { File } from "formidable";
import fs from "fs";
import pinataSDK from "@pinata/sdk";
import type { PinataPin } from "@pinata/sdk";
import { NextApiRequest, NextApiResponse } from "next";

const pinata = new pinataSDK({ pinataJWTKey: process.env.PINATA_JWT });

export const config = {
  api: {
    bodyParser: false,
  },
};

const saveFile = async (file: File) => {
  try {
    const stream = fs.createReadStream(file.filepath);
    const options = {
      pinataMetadata: {
        name: file.originalFilename,
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
      const form = formidable({ multiples: true, keepExtensions: true });
      form.parse(req, async (err, _fields, files) => {
        if (err || !files.file || files.file.length === 0) {
          console.error({ err });
          return res.status(500).send("Upload Error");
        }
        const response = await saveFile(files.file[0]);
        const { IpfsHash } = response;

        return res.send(IpfsHash);
      });
    } else if (req.method === "GET") {
        const hashContains = JSON.stringify(req.query) === JSON.stringify({}) ? undefined : req.query['cid'] as string;
        // TODO: support pagination for an explore page
        const nftData = await pinata.pinList({ hashContains });
        res.json(nftData.rows);
    }
  } catch (error) {
    console.error(error);
    res.status(500).send("Server Error");
  }
}
