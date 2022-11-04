import { spawn } from "child_process";

export const buildContract = async (path: string) => {
  return new Promise((resolve, reject) => {
    const forcBuild = spawn("forc", ["build", "-p", path], {
      stdio: "ignore",
    });
    forcBuild.on("exit", (code) => {
      if (!code) return resolve(code);
      forcBuild.kill();
      reject(new Error(`There is an error while building the "${ path }" contract, please try manually once`));
    });
  });
};
