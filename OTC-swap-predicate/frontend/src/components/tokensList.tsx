import { FC } from "react";
import { CoinQuantity } from "fuels";
import { Button, Stack } from "@fuel-ui/react";

type TokenListProps = {
    tokensFound: CoinQuantity[],
}

const TokenList: FC<TokenListProps> = ({tokensFound}: TokenListProps) => {

    // TODO Spend the tokens found at the predicate address
    async function handleTake() {
      window.alert("Taking offer not implemented yet!");
    }
  
    // TODO Recover the tokens found at the predicate address (if owner)
    async function handleCancel() {
      window.alert("Cancelling offer not implemented yet!");
    }

  return (
        <>
          <table className="App-tokenTable">
            <thead>
              <tr key="headers">
                <th>Offered Asset ID</th>
                <th>Offered Amount</th>
              </tr>
            </thead>
            <tbody>
              {tokensFound.map((token) => (
                <tr key="items">
                  <td className="App-address">{token.assetId}</td>
                  <td>{token.amount.formatUnits()}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <Stack css={{ maxW: "400px" }}>
            <Button onPress={handleTake}> Take offer </Button>
            <Button onPress={handleCancel}> Cancel offer </Button>
          </Stack>
      </>
        
        )
    }

export default TokenList;
