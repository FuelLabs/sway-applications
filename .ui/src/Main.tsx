// @ts-nocheck
import Components from "./Components";

function Main() {
  const components = Components();

  return (
    <>
      {components.map((element, index) => (
        <div key={index}>{element}</div>
      ))}
    </>
  );
}

export default Main;
