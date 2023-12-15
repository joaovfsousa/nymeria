import { MeetingStatus, OnChangeMeetingStatusCallback } from "./types";

export async function start(callback: OnChangeMeetingStatusCallback) {
  setTimeout(() => {
    start(callback);
  }, 500);
}
