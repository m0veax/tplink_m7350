import { createContext } from "react";

export type Val = 0 | 1;

type ValRow = [Val];
export type Sprite = [ValRow];

const SpriteContext = createContext<Sprite | null>(null);

export default SpriteContext;
