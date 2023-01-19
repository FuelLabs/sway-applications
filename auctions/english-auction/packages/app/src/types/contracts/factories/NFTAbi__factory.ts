/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import type { Provider, BaseWalletLocked, AbstractAddress } from 'fuels';
import { Interface, Contract } from 'fuels';
import type { NFTAbi, NFTAbiInterface } from '../NFTAbi';
const _abi = {
  types: [
    {
      typeId: 0,
      type: '()',
      components: [],
      typeParameters: null,
    },
    {
      typeId: 1,
      type: 'b256',
      components: null,
      typeParameters: null,
    },
    {
      typeId: 2,
      type: 'bool',
      components: null,
      typeParameters: null,
    },
    {
      typeId: 3,
      type: 'enum AccessError',
      components: [
        {
          name: 'OwnerDoesNotExist',
          type: 0,
          typeArguments: null,
        },
        {
          name: 'SenderNotOwner',
          type: 0,
          typeArguments: null,
        },
        {
          name: 'SenderNotOwnerOrApproved',
          type: 0,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 4,
      type: 'enum Identity',
      components: [
        {
          name: 'Address',
          type: 8,
          typeArguments: null,
        },
        {
          name: 'ContractId',
          type: 10,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 5,
      type: 'enum InputError',
      components: [
        {
          name: 'TokenAlreadyExists',
          type: 0,
          typeArguments: null,
        },
        {
          name: 'TokenDoesNotExist',
          type: 0,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 6,
      type: 'enum Option',
      components: [
        {
          name: 'None',
          type: 0,
          typeArguments: null,
        },
        {
          name: 'Some',
          type: 7,
          typeArguments: null,
        },
      ],
      typeParameters: [7],
    },
    {
      typeId: 7,
      type: 'generic T',
      components: null,
      typeParameters: null,
    },
    {
      typeId: 8,
      type: 'struct Address',
      components: [
        {
          name: 'value',
          type: 1,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 9,
      type: 'struct ApprovalEvent',
      components: [
        {
          name: 'approved',
          type: 6,
          typeArguments: [
            {
              name: '',
              type: 4,
              typeArguments: null,
            },
          ],
        },
        {
          name: 'owner',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 10,
      type: 'struct ContractId',
      components: [
        {
          name: 'value',
          type: 1,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 11,
      type: 'struct MintEvent',
      components: [
        {
          name: 'owner',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 12,
      type: 'struct OperatorEvent',
      components: [
        {
          name: 'approved',
          type: 2,
          typeArguments: null,
        },
        {
          name: 'operator',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'owner',
          type: 4,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 13,
      type: 'struct TransferEvent',
      components: [
        {
          name: 'from',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'sender',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'to',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      typeParameters: null,
    },
    {
      typeId: 14,
      type: 'u64',
      components: null,
      typeParameters: null,
    },
  ],
  functions: [
    {
      inputs: [
        {
          name: 'approved_identity',
          type: 6,
          typeArguments: [
            {
              name: '',
              type: 4,
              typeArguments: null,
            },
          ],
        },
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      name: 'approve',
      output: {
        name: '',
        type: 0,
        typeArguments: null,
      },
    },
    {
      inputs: [
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      name: 'approved',
      output: {
        name: '',
        type: 6,
        typeArguments: [
          {
            name: '',
            type: 4,
            typeArguments: null,
          },
        ],
      },
    },
    {
      inputs: [
        {
          name: 'owner',
          type: 4,
          typeArguments: null,
        },
      ],
      name: 'balance_of',
      output: {
        name: '',
        type: 14,
        typeArguments: null,
      },
    },
    {
      inputs: [
        {
          name: 'operator',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'owner',
          type: 4,
          typeArguments: null,
        },
      ],
      name: 'is_approved_for_all',
      output: {
        name: '',
        type: 2,
        typeArguments: null,
      },
    },
    {
      inputs: [
        {
          name: 'amount',
          type: 14,
          typeArguments: null,
        },
        {
          name: 'to',
          type: 4,
          typeArguments: null,
        },
      ],
      name: 'mint',
      output: {
        name: '',
        type: 0,
        typeArguments: null,
      },
    },
    {
      inputs: [
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      name: 'owner_of',
      output: {
        name: '',
        type: 6,
        typeArguments: [
          {
            name: '',
            type: 4,
            typeArguments: null,
          },
        ],
      },
    },
    {
      inputs: [
        {
          name: 'approval',
          type: 2,
          typeArguments: null,
        },
        {
          name: 'operator',
          type: 4,
          typeArguments: null,
        },
      ],
      name: 'set_approval_for_all',
      output: {
        name: '',
        type: 0,
        typeArguments: null,
      },
    },
    {
      inputs: [],
      name: 'tokens_minted',
      output: {
        name: '',
        type: 14,
        typeArguments: null,
      },
    },
    {
      inputs: [
        {
          name: 'to',
          type: 4,
          typeArguments: null,
        },
        {
          name: 'token_id',
          type: 14,
          typeArguments: null,
        },
      ],
      name: 'transfer',
      output: {
        name: '',
        type: 0,
        typeArguments: null,
      },
    },
  ],
  loggedTypes: [
    {
      logId: 0,
      loggedType: {
        name: '',
        type: 5,
        typeArguments: [],
      },
    },
    {
      logId: 1,
      loggedType: {
        name: '',
        type: 3,
        typeArguments: [],
      },
    },
    {
      logId: 2,
      loggedType: {
        name: '',
        type: 9,
        typeArguments: [],
      },
    },
    {
      logId: 3,
      loggedType: {
        name: '',
        type: 5,
        typeArguments: [],
      },
    },
    {
      logId: 4,
      loggedType: {
        name: '',
        type: 11,
        typeArguments: [],
      },
    },
    {
      logId: 5,
      loggedType: {
        name: '',
        type: 12,
        typeArguments: [],
      },
    },
    {
      logId: 6,
      loggedType: {
        name: '',
        type: 5,
        typeArguments: [],
      },
    },
    {
      logId: 7,
      loggedType: {
        name: '',
        type: 3,
        typeArguments: [],
      },
    },
    {
      logId: 8,
      loggedType: {
        name: '',
        type: 13,
        typeArguments: [],
      },
    },
  ],
  messagesTypes: [],
};

export class NFTAbi__factory {
  static readonly abi = _abi;
  static createInterface(): NFTAbiInterface {
    return new Interface(_abi) as unknown as NFTAbiInterface;
  }
  static connect(
    id: string | AbstractAddress,
    walletOrProvider: BaseWalletLocked | Provider
  ): NFTAbi {
    return new Contract(id, _abi, walletOrProvider) as unknown as NFTAbi;
  }
}
