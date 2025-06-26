import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import NumberScroller from "./components/NumberScroller";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [earnedDay, setEarnedDay] = useState("");

  const init = async () => {
    setInterval(async () => {
      const data = (await invoke("get_earned_day")) as string;
      setEarnedDay(data);
    }, 1000);
  };
  const addEventListener = async () => {
    const unlisten = await listen("infos", (event) => {
      console.log("收到下载进度事件:", event);
      // event.payload 就是从后端传来的 i （数字或对象）
    });
  };
  useEffect(() => {
    addEventListener();
  }, []);

  return (
    <main className="container">
      {earnedDay}
      {/* <NumberScroller
        before="今日上班费:"
        after=""
        number={Number(earnedDay).toFixed(2)}
      /> */}
    </main>
  );
}

export default App;
