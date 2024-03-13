/* eslint-disable no-console */
import { execa } from "execa";
import fs from "node:fs";
import { join } from "path";

const ROOT_PATH = process.cwd();
const DIST_FOLDER = join(ROOT_PATH, "./dist");
const TICTACTOE_APP_PATH = "/tictactoe/";

function setEnvVar(key, value) {
    process.env[key] = process.env[key] || value;
}

// We need to set a path to the build and a url for each frontend
export function setEnv() {
    // Website urls
    setEnvVar('BASE_URL', '');
    setEnvVar("TICTACTOE_URL", TICTACTOE_APP_PATH);
    // Dist folders
    setEnvVar("TICTACTOE_DIST", join(DIST_FOLDER, TICTACTOE_APP_PATH));
    // TODO do we need to set next env vars?

    // Log dist folders
    console.log("Output dist folders:");
    console.log("TICTACTOE_DIST", process.env.TICTACTOE_DIST);
    // Log env vars
    console.log("Output urls:");
    console.log("BASE_URL", process.env.BASE_URL);
    console.log("TICTACTOE_URL", process.env.TICTACTOE_URL);
}

export async function runPnpmCommand(commands) {
    await execa('pnpm', commands, { stdout: 'inherit' });
}

export async function buildWebsite() {
    fs.rmSync(DIST_FOLDER, { recursive: true, force: true });
    await runPnpmCommand(["build:all", "--force", "--no-cache"]);
}

