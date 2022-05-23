import React from "react";
import "../App.css";

function Square({ value, chooseSquare }) {
  return (
    <div className="square" onClick={chooseSquare}>
      {value}
    </div>
  );
}

export default Square;
