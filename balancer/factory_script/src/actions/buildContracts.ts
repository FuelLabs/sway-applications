import type { Config } from "../config";
import { buildContract } from "./buildContract";
import { buildTypes } from "./buildTypes";

export const buildContracts = async (config: Config) => {
    console.log("Building Contracts..")
    for (const { path } of config.contracts) {
        await buildContract(path);
    }

    await buildTypes(config);
}