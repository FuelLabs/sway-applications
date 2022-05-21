import "./App.css";
import { useState, useEffect } from "react";
import Square from "./Components/Square";
import { Patterns } from "./Patterns";

function App() {
  // board
  const emptySquare = "";
  const emptyBoard = Array(9).fill(emptySquare);
  const [board, setBoard] = useState(emptyBoard);

  // players
  const playerX = "X";
  const playerO = "O";
  const [player, setPlayer] = useState("start");
  const getPlayer = () => {
    if (player === "start" || player === playerO) {
      setPlayer(playerX);
    } else {
      setPlayer(playerO);
    }
  };
  const returnPlayer = (square) => {
    setBoard(
      board.map((value, index) => {
        if (index === square && value === emptySquare) {
          return player;
        }

        return value;
      })
    );
  };

  // patterns
  const [result, setResult] = useState({ victor: "none", state: "none" });
  const findWinningPattern = () => {
    Patterns.forEach((pattern) => {
      const currentPattern = board[pattern[0]];
      if (currentPattern === emptySquare) return;
      let foundWinningPattern = true;

      pattern.forEach((index) => {
        if (board[index] !== currentPattern) {
          foundWinningPattern = false;
        }
      });

      if (foundWinningPattern) {
        setResult({ victor: player, state: "victory" });
      }
    });
  };
  const findTiePattern = () => {
    let filled = true;
    board.forEach((square) => {
      if (square === emptySquare) {
        filled = false;
      }
    });
    if (filled) {
      setResult({ victor: "none", state: "tie" });
    }
  };
  const findPattern = () => {
    findWinningPattern();
    findTiePattern();
  };

  // game results
  const resetGame = () => {
    setBoard(emptyBoard);
    setPlayer("start");
  };
  useEffect(() => {
    getPlayer();
    findPattern();
    // eslint-disable-next-line
  }, [board]);
  useEffect(() => {
    if (result.state === "victory") {
      alert(`Congratulations player ${result.victor}! You've won! 🚀`);
      resetGame();
    } else if (result.state === "tie") {
      alert("It's a tie... Best 2 out of 3? 🎮");
      resetGame();
    }
    // eslint-disable-next-line
  }, [result]);

  return (
    <div className="App">
      <div className="header">
        <img
          id="header-logo"
          src="data:image/png;base64,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"
          alt="Fuel Logo"
        />
      </div>
      <h1>🌴 Sway-Tac-Toe</h1>
      <div className="board">
        <div className="row">
          <Square
            value={board[0]}
            chooseSquare={() => {
              returnPlayer(0);
            }}
          />
          <Square
            value={board[1]}
            chooseSquare={() => {
              returnPlayer(1);
            }}
          />
          <Square
            value={board[2]}
            chooseSquare={() => {
              returnPlayer(2);
            }}
          />
        </div>
        <div className="row">
          <Square
            value={board[3]}
            chooseSquare={() => {
              returnPlayer(3);
            }}
          />
          <Square
            value={board[4]}
            chooseSquare={() => {
              returnPlayer(4);
            }}
          />
          <Square
            value={board[5]}
            chooseSquare={() => {
              returnPlayer(5);
            }}
          />
        </div>
        <div className="row">
          <Square
            value={board[6]}
            chooseSquare={() => {
              returnPlayer(6);
            }}
          />
          <Square
            value={board[7]}
            chooseSquare={() => {
              returnPlayer(7);
            }}
          />
          <Square
            value={board[8]}
            chooseSquare={() => {
              returnPlayer(8);
            }}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
