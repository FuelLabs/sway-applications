import { randomBytes, hexlify } from 'fuels';

// eslint-disable-next-line no-console
console.log(`Please add to your '.env' file:
PRIVATE_KEY="${hexlify(randomBytes(32))}"
`);
