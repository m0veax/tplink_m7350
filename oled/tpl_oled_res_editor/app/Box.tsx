import { FC, memo, useContext, useEffect, useState } from "react";
import SpritContext from "./SpriteContext";

const Box: FC<{ r: number; c: number }> = ({ r, c }) => {
  const { sprite, setChecked } = useContext(SpritContext);
  const [mouseDown, setMouseDown] = useState(false);

  const checked = sprite[r][c] == 1;

  const swap = (e) => {
    setChecked(c, r);
  };

  const md = () => { setMouseDown(true); };
  const mu = () => { setMouseDown(false); };

  useEffect(() => {
    document.body.addEventListener("mousedown", md);
    document.body.addEventListener("mouseup", mu);

    return () => {
      document.body.removeEventListener("mousedown", md);
      document.body.removeEventListener("mouseup", mu);
    };
  }, []);

  const me = (e) => {
    e.preventDefault();
    e.stopPropagation();
    if (mouseDown) {
      swap();
    }
  };

  const color = checked ? "blue" : "white";

  return (
    <div
      style={{ width: 8, height: 8, background: color }}
      onMouseDown={swap}
      onMouseEnter={me}
    />
  );
};

export default Box;
