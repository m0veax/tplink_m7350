import { memo } from "react";
import Box from "./Box";

const Row = memo(({ children }) => (
  <div style={{ display: "flex" }}>
    {children}
  </div>
));

const SpriteRows = ({ children }) => (
  <div style={{ display: "flex", flexDirection: "column", border: "4px solid #3f3" }}>
    {children}
  </div>
);

const SpriteMap = ({ width, height }) => {
  const rows = [];
  for (let r = 0; r < height; r++) {
    const cols = [];
    for (let c = 0; c < width; c++) {
      cols.push(<Box key={`${r}_${c}`} {...{r, c}} />);
    }
    rows.push(<Row key={r}>{cols}</Row>);
  }

  return <SpriteRows>{rows}</SpriteRows>;
};

export default SpriteMap;
