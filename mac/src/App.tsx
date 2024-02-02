import { invoke } from "@tauri-apps/api/tauri";
import { MenuItem } from "./components/MenuItem";
import { Separator } from "./components/Separator";
import { StateChanger } from "./components/StateChanger";
import { State } from "./domain/state";

function quitHandler() {
  invoke("quit");
}

function App() {
  return (
    <div className="text-white">
      <div className="flex flex-row justify-evenly my-3">
        <StateChanger state={State.Free} />
        <StateChanger state={State.Maybe} />
        <StateChanger state={State.Busy} />
      </div>
      <Separator />
      <MenuItem title="Quit" onClick={quitHandler} />
    </div>
  );
}

export default App;
