import pinataSDK from "@pinata/sdk";
import { NextApiRequest, NextApiResponse } from "next";

const pinata = new pinataSDK({ pinataJWTKey: process.env.PINATA_JWT });

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    const metadata = JSON.parse(req.query.filter as string);
    // TODO: support pagination for an explore page
    const nftData = await pinata.pinList({ metadata, status: "pinned" });
    res.json(nftData.rows);
  }
}
