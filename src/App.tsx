import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [commandResult, setCommandResult] = createSignal("");
  const [name, setName] = createSignal("");

  async function command() {
    setCommandResult(await invoke("greet", { name: name() }));
  }

  return (
    <div class="container">
      <h1>CD Drive Controller</h1>
      <p>Please input command.</p>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          command();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
        />
        <button type="submit">Greet</button>
      </form>

      <p>{commandResult()}</p>
    </div>
  );
}

export default App;
