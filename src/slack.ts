import { App, UserHuddleChangedEvent } from "@slack/bolt";
import { MeetingStatus, OnChangeMeetingStatusCallback } from "./types";

interface SlackUserHuddleChangedEventPayload {
  event: UserHuddleChangedEvent;
}

enum HuddleState {
  ON = "in_a_huddle",
  OFF = "default_unset",
}

export async function start(callback: OnChangeMeetingStatusCallback) {
  const app = new App({
    token: process.env.SLACK_BOT_TOKEN,
    signingSecret: process.env.SLACK_SIGNING_SECRET,
    socketMode: true,
    appToken: process.env.SLACK_APP_TOKEN,
  });

  app.event(new RegExp("user_huddle_changed"), async (payload) => {
    const { event } = payload as SlackUserHuddleChangedEventPayload;
    const huddleState = event.user.profile.huddle_state;

    if (event.user.id !== process.env.SLACK_USER_ID) {
      return;
    }

    if (huddleState === HuddleState.ON) {
      callback(MeetingStatus.Busy);
    } else if (huddleState === HuddleState.OFF) {
      callback(MeetingStatus.Free);
    } else {
      console.warn("Unknown huddle state: ", huddleState);
    }
  });

  await app.start();

  process.on("SIGINT", async () => {
    await app.stop();
    console.log("Slack app stopped");
  });

  console.log("Slack app is running!");
}
