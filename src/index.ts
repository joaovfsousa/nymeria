import { config } from "dotenv";
config({
  override: true,
});

import { start as startSlackIntegration } from "./slack";
import { MeetingStatus } from "./types";

function changeMeetingStatusCallback(status: MeetingStatus) {
  console.log("Status changed to: ", status);
}

(async () => {
  await startSlackIntegration(changeMeetingStatusCallback);
})();
