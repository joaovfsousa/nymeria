import { invoke } from "@tauri-apps/api/tauri";
import { MenuItem } from "./components/MenuItem";
import { Separator } from "./components/Separator";

enum State {
  Free = "free",
  Maybe = "maybe",
  Busy = "busy",
}

function quitHandler() {
  invoke("quit");
}

function getSetStateHandler(state: State) {
  return function () {
    console.log("aqui");
    invoke("set_device_state", { deviceId: "nymeria_app", state });
  };
}

function App() {
  return (
    <div className="text-white">
      <MenuItem title="Settings" onClick={getSetStateHandler(State.Free)} />
      <Separator />
      <MenuItem title="Quit" onClick={quitHandler} />
    </div>
  );
}

export default App;
