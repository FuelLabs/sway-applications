import { FC } from "react";
import { CoinQuantity } from "fuels";

type TokenListProps = {
    tokensFound: CoinQuantity[],
}

const TokenList: FC<TokenListProps> = ({tokensFound}: TokenListProps) => {

    // TODO Spend the tokens found at the predicate address
    async function handleTake() {
      window.alert("Not implemented yet!");
    }
  
    // TODO Recover the tokens found at the predicate address (if owner)
    async function handleCancel() {
      window.alert("Not implemented yet!");
    }
  return (
        <>
          <p>Offer found :</p>
          <table className="App-tokenTable">
            <thead>
              <tr key="headers">
                <th>Asset ID</th>
                <th>Amount</th>
              </tr>
            </thead>
            <tbody>
              {tokensFound.map((token) => (
                <tr key="items">
                  <td className="App-address">{token.assetId}</td>
                  <td>{token.amount.toString()}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <div>
            <button className="App-button" onClick={handleTake}>Take offer</button>
            <button className="App-button" onClick={handleCancel}>Cancel offer</button>
          </div>
      </>
        
        )
    }

export default TokenList;
