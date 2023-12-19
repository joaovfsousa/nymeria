import { MeetingStatus } from "./types";

let lastStatus: MeetingStatus;

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

    return grepStdout.length ? MeetingStatus.Busy : MeetingStatus.Free;
  } catch (e) {
    return MeetingStatus.Free;
  }
}

async function loop() {
  const newStatus = await getCurrentStatus();

  if (newStatus !== lastStatus) {
    console.log(`${new Date().toISOString()}: Current status: ${newStatus}`);

    try {
      await fetch("http://192.168.0.150:80/change-status", {
        method: "POST",
        headers: {
          contentType: "text/plain",
        },
        body: newStatus,
      });
    } catch (e) {
      console.error(
        `${new Date().toISOString()}: Error on update esp32: ${JSON.stringify(
          e,
        )}`,
      );
    }

    lastStatus = newStatus;
  }
}

(async () => {
  while (1) {
    await loop();
  }
})();
