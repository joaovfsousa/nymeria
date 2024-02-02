import { FC } from "react";
import "./index.css";
import { invoke } from "@tauri-apps/api/tauri";
import { State } from "../../domain/state";

type Color = "red" | "yellow" | "green";

type StateChangerProps = {
  state: State;
};

const stateToColor: Record<State, Color> = {
  [State.Free]: "green",
  [State.Maybe]: "yellow",
  [State.Busy]: "red",
};

function getSetStateHandler(state: State) {
  return function () {
    invoke("set_device_state", { deviceId: "nymeria_app", state });
  };
}

export const StateChanger: FC<StateChangerProps> = ({ state }) => {
  return (
    <div
      className={`circle circle--${stateToColor[state]}`}
      onClick={getSetStateHandler(state)}
    ></div>
  );
};
