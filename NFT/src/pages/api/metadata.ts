import { NextApiRequest, NextApiResponse } from "next";
import pinataSDK from "@pinata/sdk";

const pinata = new pinataSDK({ pinataJWTKey: process.env.PINATA_JWT });

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
    if (req.method === "PUT") {
        const { ipfsHash, metadata } = JSON.parse(req.body);

        const response = await pinata.hashMetadata(ipfsHash, metadata);
        return res.send(response);
    }
}
