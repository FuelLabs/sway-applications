export const contracts = {
  'nft-contract': {
    abi: {
  "encoding": "1",
  "types": [
    {
      "typeId": 0,
      "type": "()",
      "components": [],
      "typeParameters": null
    },
    {
      "typeId": 1,
      "type": "b256",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 2,
      "type": "bool",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 3,
      "type": "enum AccessError",
      "components": [
        {
          "name": "NotOwner",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 4,
      "type": "enum BurnError",
      "components": [
        {
          "name": "NotEnoughCoins",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 5,
      "type": "enum Identity",
      "components": [
        {
          "name": "Address",
          "type": 15,
          "typeArguments": null
        },
        {
          "name": "ContractId",
          "type": 18,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 6,
      "type": "enum InitializationError",
      "components": [
        {
          "name": "CannotReinitialized",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 7,
      "type": "enum Metadata",
      "components": [
        {
          "name": "B256",
          "type": 1,
          "typeArguments": null
        },
        {
          "name": "Bytes",
          "type": 17,
          "typeArguments": null
        },
        {
          "name": "Int",
          "type": 22,
          "typeArguments": null
        },
        {
          "name": "String",
          "type": 21,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 8,
      "type": "enum MintError",
      "components": [
        {
          "name": "CannotMintMoreThanOneNFTWithSubId",
          "type": 0,
          "typeArguments": null
        },
        {
          "name": "MaxNFTsMinted",
          "type": 0,
          "typeArguments": null
        },
        {
          "name": "NFTAlreadyMinted",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 9,
      "type": "enum Option",
      "components": [
        {
          "name": "None",
          "type": 0,
          "typeArguments": null
        },
        {
          "name": "Some",
          "type": 13,
          "typeArguments": null
        }
      ],
      "typeParameters": [
        13
      ]
    },
    {
      "typeId": 10,
      "type": "enum PauseError",
      "components": [
        {
          "name": "Paused",
          "type": 0,
          "typeArguments": null
        },
        {
          "name": "NotPaused",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 11,
      "type": "enum SetError",
      "components": [
        {
          "name": "ValueAlreadySet",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 12,
      "type": "enum State",
      "components": [
        {
          "name": "Uninitialized",
          "type": 0,
          "typeArguments": null
        },
        {
          "name": "Initialized",
          "type": 5,
          "typeArguments": null
        },
        {
          "name": "Revoked",
          "type": 0,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 13,
      "type": "generic T",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 14,
      "type": "raw untyped ptr",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 15,
      "type": "struct Address",
      "components": [
        {
          "name": "bits",
          "type": 1,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 16,
      "type": "struct AssetId",
      "components": [
        {
          "name": "bits",
          "type": 1,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 17,
      "type": "struct Bytes",
      "components": [
        {
          "name": "buf",
          "type": 20,
          "typeArguments": null
        },
        {
          "name": "len",
          "type": 22,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 18,
      "type": "struct ContractId",
      "components": [
        {
          "name": "bits",
          "type": 1,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 19,
      "type": "struct OwnershipSet",
      "components": [
        {
          "name": "new_owner",
          "type": 5,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 20,
      "type": "struct RawBytes",
      "components": [
        {
          "name": "ptr",
          "type": 14,
          "typeArguments": null
        },
        {
          "name": "cap",
          "type": 22,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 21,
      "type": "struct String",
      "components": [
        {
          "name": "bytes",
          "type": 17,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 22,
      "type": "u64",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 23,
      "type": "u8",
      "components": null,
      "typeParameters": null
    }
  ],
  "functions": [
    {
      "inputs": [
        {
          "name": "_asset",
          "type": 16,
          "typeArguments": null
        }
      ],
      "name": "decimals",
      "output": {
        "name": "",
        "type": 9,
        "typeArguments": [
          {
            "name": "",
            "type": 23,
            "typeArguments": null
          }
        ]
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns the number of decimals the asset uses."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Additional Information"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " The standardized decimals for NFTs is 0u8."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to query the decimals."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [Option<u8>] - The decimal precision used by `asset`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId, asset: AssedId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let decimals = contract_abi.decimals(asset).unwrap();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(decimals == 0u8);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        }
      ],
      "name": "name",
      "output": {
        "name": "",
        "type": 9,
        "typeArguments": [
          {
            "name": "",
            "type": 21,
            "typeArguments": null
          }
        ]
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns the name of the asset, such as “Ether”."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to query the name."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [Option<String>] - The name of `asset`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use std::string::String;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_ic: ContractId, asset: AssetId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let name = contract_abi.name(asset).unwrap();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(name.len() != 0);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        }
      ],
      "name": "symbol",
      "output": {
        "name": "",
        "type": 9,
        "typeArguments": [
          {
            "name": "",
            "type": 21,
            "typeArguments": null
          }
        ]
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns the symbol of the asset, such as “ETH”."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to query the symbol."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [Option<String>] - The symbol of `asset`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use std::string::String;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId, asset: AssetId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let symbol = contract_abi.symbol(asset).unwrap();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(symbol.len() != 0);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "total_assets",
      "output": {
        "name": "",
        "type": 22,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns the total number of individual NFTs for this contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [u64] - The number of assets that this contract has minted."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let total_assets = contract_abi.total_assets();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(total_assets != 0);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        }
      ],
      "name": "total_supply",
      "output": {
        "name": "",
        "type": 9,
        "typeArguments": [
          {
            "name": "",
            "type": 22,
            "typeArguments": null
          }
        ]
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns the total supply of coins for an asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Additional Information"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " This must always be at most 1 for NFTs."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to query the total supply."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [Option<u64>] - The total supply of coins for `asset`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId, asset: AssetId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let total_supply = contract_abi.total_supply(asset).unwrap();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(total_supply == 1);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "sub_id",
          "type": 1,
          "typeArguments": null
        },
        {
          "name": "amount",
          "type": 22,
          "typeArguments": null
        }
      ],
      "name": "burn",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Burns assets sent with the given `sub_id`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Additional Information"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " NOTE: The sha-256 hash of `(ContractId, SubId)` must match the `AssetId` where `ContractId` is the id of"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " the implementing contract and `SubId` is the given `sub_id` argument."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `sub_id`: [SubId] - The sub-identifier of the asset to burn."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `amount`: [u64] - The quantity of coins to burn."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the contract is paused."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src3::SRC3;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId, asset_id: AssetId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SR3, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     contract_abi.burn {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "         gas: 10000,"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "         coins: 1,"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "         asset_id: AssetId,"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     } (ZERO_B256, 1);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "payable",
          "arguments": []
        },
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "recipient",
          "type": 5,
          "typeArguments": null
        },
        {
          "name": "sub_id",
          "type": 1,
          "typeArguments": null
        },
        {
          "name": "amount",
          "type": 22,
          "typeArguments": null
        }
      ],
      "name": "mint",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Mints new assets using the `sub_id` sub-identifier."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Additional Information"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " This conforms to the SRC-20 NFT portion of the standard for a maximum"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " mint amount of 1 coin per asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `recipient`: [Identity] - The user to which the newly minted assets are transferred to."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `sub_id`: [SubId] - The sub-identifier of the newly minted asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `amount`: [u64] - The quantity of coins to mint."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the contract is paused."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When amount is greater than one."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the asset has already been minted."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When more than the MAX_SUPPLY NFTs have been minted."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `3`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `2`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src3::SRC3;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SR3, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     contract_abi.mint(Identity::ContractId(ContractId::this()), ZERO_B256, 1);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        },
        {
          "name": "key",
          "type": 21,
          "typeArguments": null
        }
      ],
      "name": "metadata",
      "output": {
        "name": "",
        "type": 9,
        "typeArguments": [
          {
            "name": "",
            "type": 7,
            "typeArguments": null
          }
        ]
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns metadata for the corresponding `asset` and `key`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to query the metadata."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `key`: [String] - The key to the specific metadata."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [Option<Metadata>] - `Some` metadata that corresponds to the `key` or `None`."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src_7::{SRC7, Metadata};"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use std::string::String;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId, asset: AssetId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let contract_abi = abi(SRC7, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let key = String::from_ascii_str(\"image\");"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let data = contract_abi.metadata(asset, key);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(data.is_some());"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "owner",
      "output": {
        "name": "",
        "type": 12,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns the owner."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Return Values"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [State] - Represents the state of ownership for this contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use standards::src5::SRC5;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let ownership_abi = abi(contract_id, SRC_5);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     match ownership_abi.owner() {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "         State::Uninitalized => log(\"The ownership is uninitalized\"),"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "         State::Initialized(owner) => log(\"The ownership is initalized\"),"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "         State::Revoked => log(\"The ownership is revoked\"),"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "_asset",
          "type": 16,
          "typeArguments": null
        },
        {
          "name": "_decimals",
          "type": 23,
          "typeArguments": null
        }
      ],
      "name": "set_decimals",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " This function should never be called."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Additional Information"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " NFT decimals are always `0u8` and thus must not be set."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " This function is an artifact of the SetAssetAttributes ABI definition,"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " but does not have a use in this contract as the decimal value is hardcoded."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the function is called."
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        },
        {
          "name": "name",
          "type": 21,
          "typeArguments": null
        }
      ],
      "name": "set_name",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Sets the name of an asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to set the name."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `name`: [String] - The name of the asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the caller is not the owner of the contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the name has already been set for an asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `2`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use asset::SetAssetAttributes;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use std::string::String;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(asset: AssetId, contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let set_abi = abi(SetAssetAttributes, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let src_20_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let name = String::from_ascii_str(\"Ether\");"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     set_abi.set_name(asset, name);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(src_20_abi.name(asset) == name);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        },
        {
          "name": "symbol",
          "type": 21,
          "typeArguments": null
        }
      ],
      "name": "set_symbol",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Sets the symbol of an asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset of which to set the symbol."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `symbol`: [String] - The symbol of the asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the caller is not the owner of the contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the symbol has already been set for an asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `2`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use asset::SetAssetAttributes;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src20::SRC20;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use std::string::String;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(asset: AssetId, contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let set_abi = abi(SetAssetAttributes, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let src_20_abi = abi(SRC20, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let symbol = String::from_ascii_str(\"ETH\");"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     set_abi.set_symbol(asset, symbol);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(src_20_abi.symbol(asset) == symbol);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset",
          "type": 16,
          "typeArguments": null
        },
        {
          "name": "key",
          "type": 21,
          "typeArguments": null
        },
        {
          "name": "metadata",
          "type": 7,
          "typeArguments": null
        }
      ],
      "name": "set_metadata",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Stores metadata for a specific asset and key pair."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `asset`: [AssetId] - The asset for the metadata to be stored."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `key`: [String] - The key for the metadata to be stored."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `metadata`: [Metadata] - The metadata to be stored."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the metadata has already been set for an asset."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `2`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Example"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use asset::metdata::SetAssetMetadata;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use src_7::{SRC7, Metadata};"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(asset: AssetId, key: String, contract_id: ContractId, metadata: Metadata) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let set_abi = abi(SetAssetMetadata, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let src_7_abi = abi(SRC7, contract);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     set_abi.set_metadata(storage.metadata, asset, key, metadata);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(src_7_abi.metadata(asset, key) == metadata);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "is_paused",
      "output": {
        "name": "",
        "type": 2,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Returns whether the contract is paused."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Returns"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * [bool] - The pause state for the contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use sway_libs::pausable::Pausable;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let pausable_abi = abi(Pausable, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(!pausable_abi.is_paused());"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "pause",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Pauses the contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the caller is not the contract owner."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use sway_libs::pausable::Pausable;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let pausable_abi = abi(Pausable, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     pausable_abi.pause();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(pausable_abi.is_paused());"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "unpause",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Unpauses the contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When the caller is not the contract owner."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Accesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Writes: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use sway_libs::pausable::Pausable;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract_id: ContractId) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let pausable_abi = abi(Pausable, contract_id);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     pausable_abi.unpause();"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(!pausable_abi.is_paused());"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "owner",
          "type": 5,
          "typeArguments": null
        }
      ],
      "name": "constructor",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "doc-comment",
          "arguments": [
            " Sets the defaults for the contract."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Arguments"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * `owner`: [Identity] - The `Identity` that will be the first owner."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Reverts"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * When ownership has been set before."
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Number of Storage Acesses"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Reads: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " * Write: `1`"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " # Examples"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```sway"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use standards::src5::SRC5;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " use nft::Constructor;"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " fn foo(contract: ContractId, owner: Identity) {"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let src_5_abi = abi(SRC5, contract.bits());"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(src_5_abi.owner() == State::Uninitialized);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            ""
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     let constructor_abi = abi(Constructor, contract.bits());"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     constructor_abi.constructor(owner);"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            "     assert(src_5_abi.owner() == State::Initialized(owner));"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " }"
          ]
        },
        {
          "name": "doc-comment",
          "arguments": [
            " ```"
          ]
        },
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "max_supply",
      "output": {
        "name": "",
        "type": 22,
        "typeArguments": null
      },
      "attributes": null
    }
  ],
  "loggedTypes": [
    {
      "logId": "10032608944051208538",
      "loggedType": {
        "name": "",
        "type": 10,
        "typeArguments": []
      }
    },
    {
      "logId": "4237256875605624201",
      "loggedType": {
        "name": "",
        "type": 4,
        "typeArguments": []
      }
    },
    {
      "logId": "17188485204969729195",
      "loggedType": {
        "name": "",
        "type": 8,
        "typeArguments": []
      }
    },
    {
      "logId": "13791596350235125220",
      "loggedType": {
        "name": "",
        "type": 11,
        "typeArguments": []
      }
    },
    {
      "logId": "4571204900286667806",
      "loggedType": {
        "name": "",
        "type": 3,
        "typeArguments": []
      }
    },
    {
      "logId": "2161305517876418151",
      "loggedType": {
        "name": "",
        "type": 6,
        "typeArguments": []
      }
    },
    {
      "logId": "16280289466020123285",
      "loggedType": {
        "name": "",
        "type": 19,
        "typeArguments": []
      }
    }
  ],
  "messagesTypes": [],
  "configurables": [
    {
      "name": "MAX_SUPPLY",
      "configurableType": {
        "name": "",
        "type": 22,
        "typeArguments": null
      },
      "offset": 49536
    }
  ]
},
    bytecode: base64ToUint8Array('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'),
  },

};

function base64ToUint8Array(base64: string) {
  var binaryString = atob(base64);
  var bytes = new Uint8Array(binaryString.length);
  for (var i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}
