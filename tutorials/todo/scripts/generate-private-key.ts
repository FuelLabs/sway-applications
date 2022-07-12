import {randomBytes, hexlify} from 'fuels';

console.log(`Please add to your '.env' file:
PRIVATE_KEY="${hexlify(randomBytes(32))}"
`);
