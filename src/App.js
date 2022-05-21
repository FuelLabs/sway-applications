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
  const playerX = "❌";
  const playerO = "⭕";
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
