import { config } from "dotenv";

import { start as startSlackIntegration } from "./slack";
import { App, MeetingStatus } from "./types";
import { startHttpServer } from "./http-server";

config({
  override: true,
});

const statusByApp: Record<App, MeetingStatus> = {
  [App.Slack]: MeetingStatus.Free,
  [App.Teams]: MeetingStatus.Free,
};

function changeMeetingStatusCallback(app: App, status: MeetingStatus) {
  statusByApp[app] = status;
  console.log(`${app} status changed to: `, status);
}

async function updateTeamsStatus() {
  const execa = (await import("execa")).execa;

  const { stdout } = await execa("ioreg", ["-l"], {
    shell: true,
    stdout: "pipe",
  });

  // Now, use grep in Node.js to filter the output
  const { stdout: grepStdout } = await execa("grep", ["IOAudioEngineState"], {
    input: stdout,
    stdout: "pipe",
    shell: true,
  });

  const { stdout: finalStdout } = await execa("grep", ["1"], {
    input: grepStdout,
    shell: true,
  });

  console.log(finalStdout.length);
  // const response = await $`ioreg -l`.pipeStdout!(
  //   $({ stdin: "pipe" })`grep IOAudioEngineState`,
  // ).pipeStdout!($({ stdin: "pipe" })`grep -o '1'`);
  //
  // console.log(response.stdout.length);
}

async function getCurrentStatus() {
  await updateTeamsStatus();

  const status = Object.entries(statusByApp);
  if (status.some(([_, status]) => status === MeetingStatus.Busy)) {
    return MeetingStatus.Busy;
  }
  if (status.some(([_, status]) => status === MeetingStatus.Maybe)) {
    return MeetingStatus.Maybe;
  }
  return MeetingStatus.Free;
}

async () => {
  await startSlackIntegration(
    changeMeetingStatusCallback.bind(this, App.Slack),
  );

  startHttpServer(getCurrentStatus);
};

updateTeamsStatus();
