import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [earnedDay, setEarnedDay] = useState("");
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await invoke("my_custom_command");
    // setGreetMsg(await invoke("greet", { name }));
  }
  const init = async () => {
    setInterval(async () => {
      const data = (await invoke("get_earned_day")) as string;
      setEarnedDay(data);
    }, 1000);
  };
  useEffect(() => {
    init();
  }, []);

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{earnedDay}</p>
    </main>
  );
}

export default App;
