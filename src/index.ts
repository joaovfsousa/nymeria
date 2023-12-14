import { config } from "dotenv";
config({
  override: true,
});

import { start as startSlackIntegration } from "./slack";
import { App, MeetingStatus } from "./types";

const statusByApp: Record<App, MeetingStatus> = {
  [App.SLACK]: MeetingStatus.OFF,
  [App.TEAMS]: MeetingStatus.OFF,
};

function changeMeetingStatusCallback(app: App, status: MeetingStatus) {
  statusByApp[app] = status;
  console.log(`${app} status changed to: `, status);
}

(async () => {
  await startSlackIntegration(
    changeMeetingStatusCallback.bind(this, App.SLACK),
  );
})();
