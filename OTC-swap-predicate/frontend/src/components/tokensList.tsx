import { FC } from "react";

type TokenListProps = {
    tokensFound: {"asset_id": string, "amount": string}[],
    handleTake: () => void,
    handleCancel: () => void,
}

const TokenList: FC<TokenListProps> = ({tokensFound, handleTake, handleCancel}: TokenListProps) => {
    return (
        <>
        {tokensFound.length > 0 &&
          <>
            <p>Tokens found at address :</p>
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
                    <td className="App-address">{token.asset_id}</td>
                    <td>{token.amount}</td>
                  </tr>
                ))}
              </tbody>
            </table>

            <div>
              <button className="App-button" onClick={handleTake}>Take offer</button>
              <button className="App-button" onClick={handleCancel}>Cancel offer</button>
            </div>

          </>
        } 
      </>
        
        )
    }

export default TokenList;
