import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [recordName, setRecordName] = useState("");

  async function addRecord() {
    setGreetMsg(await invoke("add_record", { recordName }));
  }

  return (
    <div className="container">
      <h1>Tauri password manager</h1>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setRecordName(e.currentTarget.value)}
            placeholder="Enter record name"
          />
          <button type="button" onClick={() => addRecord()}>
            Add
          </button>
        </div>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
