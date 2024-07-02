import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

let code = 'export const contracts = {\n';

const outputDirectory = `${__dirname}/../NFT-contract/out/debug`;
const abiPath = `${outputDirectory}/NFT-contract-abi.json`;
const bytecodePath = `${outputDirectory}/NFT-contract.bin`;

const abi = fs.readFileSync(abiPath, 'utf8');
const bytecode = fs.readFileSync(bytecodePath);

code += `  'nft-contract': {\n`;
code += `    abi: ${abi},\n`;
code += `    bytecode: base64ToUint8Array('${bytecode.toString('base64')}'),\n`;
code += '  },\n';

code += `
};

function base64ToUint8Array(base64: string) {
  var binaryString = atob(base64);
  var bytes = new Uint8Array(binaryString.length);
  for (var i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}
`;

fs.writeFileSync(`${__dirname}/../src/generated/contract.ts`, code);
console.log('Generated');