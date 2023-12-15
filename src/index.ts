import { config } from "dotenv";

import { MeetingStatus } from "./types";
import { startHttpServer } from "./http-server";

config({
  override: true,
});

async function getCurrentStatus() {
  const execa = (await import("execa")).execa;

  const { stdout } = await execa("ioreg", ["-l"], {
    shell: true,
    stdout: "pipe",
  });

  const { stdout: grepStdout } = await execa("grep", ["IOAudioEngineState"], {
    input: stdout,
    stdout: "pipe",
    shell: true,
  });

  return grepStdout.includes("1") ? MeetingStatus.Busy : MeetingStatus.Free;
}

(() => {
  startHttpServer(getCurrentStatus);
})();
