import { toast } from '@fuel-ui/react';
import { SignatureInfoInput } from '../../../contracts/MultisigContractAbi';

export async function updateSignature(
  index: number,
  signature: string,
  handler: (signatures: SignatureInfoInput[]) => void,
  signatures: SignatureInfoInput[]
) {
  const localSignatures = [...signatures];
  // TODO: Figure out how to convert the signed message into a B512 in the SignatureInfo
  localSignatures[index].signature.bytes = [signature, signature];
  handler(localSignatures);
}

export async function addSignature(
  handler: (signatures: SignatureInfoInput[]) => void,
  signatures: SignatureInfoInput[]
) {
  let signature: SignatureInfoInput = {
    message_format: { None: [] },
    message_prefix: { None: [] },
    signature: { bytes: ['', ''] },
    wallet_type: { Fuel: [] },
  };
  handler([...signatures, signature]);
}

export async function removeSignature(
  handler: (signatures: SignatureInfoInput[]) => void,
  signatures: SignatureInfoInput[]
) {
  if (signatures.length === 1) {
    toast.error('Cannot remove the last signature');
    return;
  }

  handler([...signatures.splice(0, signatures.length - 1)]);
}
