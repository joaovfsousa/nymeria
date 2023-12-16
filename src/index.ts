import { config } from "dotenv";

import { MeetingStatus } from "./types";
import { startHttpServer } from "./http-server";

config({
  override: true,
});

let status = MeetingStatus.Free;

async function getCurrentStatus() {
  try {
    const execa = (await import("execa")).execa;

    const { stdout } = await execa("ioreg", ["-l"], {
      shell: true,
      stdout: "pipe",
    });

    const { stdout: grepStdout } = await execa(
      "grep",
      ["-o", "'IOAudioEngineState\" = 1'"],
      {
        input: stdout,
        stdout: "pipe",
        shell: true,
      },
    );

    status = grepStdout.length ? MeetingStatus.Busy : MeetingStatus.Free;
  } catch (e) {
    status = MeetingStatus.Free;
  }
}

function getStatus() {
  setTimeout(async () => {
    await getCurrentStatus();
  }, 10000);
  return status;
}

(async () => {
  await getCurrentStatus();
  startHttpServer(getStatus);
})();
