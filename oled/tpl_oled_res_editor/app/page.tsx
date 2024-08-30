"use client";
import { FC, useCallback, useEffect, useState } from "react";
import Image from "next/image";
import { useFilePicker } from "use-file-picker";
import styles from "./page.module.css";
import SpriteContext from "./SpriteContext";
import SpriteMap from "./SpriteMap";

const Editor = ({ sprite: initialSprite, width, height }) => {
  const [sprite, setSprite] = useState(initialSprite);

  const setChecked = (x, y) => {
    setSprite((s) => {
      const newr = s[y].map((e, i) => {
        if (i == x) {
          return e == 1 ? 0 : 1;
        }
        return e;
      });
      return { ...s, [y]: newr };
    });
  };

  return (
    <SpriteContext.Provider value={{sprite, setChecked}}>
      <SpriteMap width={width} height={height} />
    </SpriteContext.Provider>
  );
};


export default function Home() {
  const [fbuf, setFbuf] = useState<ArrayBuffer | null>(null);
  const [inProgress, setInProgress] = useState(false);
  const [sprite, setSprite] = useState<null | Sprite>(null);

  const { openFilePicker, filesContent, loading, errors, plainFiles } =
    useFilePicker({
      multiple: false,
      readAs: "ArrayBuffer",
      maxFileSize: 1, // megabytes
    });

  const width = 128;
  const height = 15;

  const parseSprite = async (data: Uint8Array) => {
    setSprite(null);
    setInProgress(true);
    setTimeout(async () => {
      try {
        const s = {};
        for (let row = 0; row < height; row++) {
          const r = [];
          for (let col = 0; col < width; col++) {
            const pos = row * width + col;
            const byte = Math.floor(pos / 8);
            const bit = 7 - (pos % 8);
            const b = (data[byte] >> bit) & 1;
            r.push(b);
          }
          s[row] = r;
        }
        setSprite(s);
      } catch (e) {
        console.error(e);
      } finally {
        console.info("DONE:", new Date());
        setInProgress(false);
      }
    }, 100);
  };

  const reload = useCallback(() => {
    if (fbuf) {
      parseSprite(new Uint8Array(fbuf));
    }
  }, [fbuf]);

  useEffect(() => {
      reload();
  }, [reload]);

  useEffect(() => {
    if (filesContent.length) {
      const f = filesContent[0].content;
      setFbuf(f);
    }
  }, [filesContent]);

  const fileName = plainFiles.length > 0 ? plainFiles[0].name : "";
  const pending = loading || inProgress;

  return (
    <div>
      <menu>
        <button disabled={pending} onClick={openFilePicker}>
          {pending ? "..." : "Load sprite"}
        </button>
      </menu>
      <main className={styles.main}>
        {sprite && <Editor {...{sprite, width, height}} />}
      </main>
    </div>
  );
}
