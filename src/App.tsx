import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [pong, setPong] = useState("(waiting...)");

  useEffect(() => {
    invoke<string>("ping").then(setPong);
  }, []);

  return <div>Backend says: {pong}</div>;
}

export default App;
