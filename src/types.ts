export enum MeetingStatus {
  Busy = "busy",
  Maybe = "maybe",
  Free = "free",
}

export enum App {
  Slack = "Slack",
  Teams = "Teams",
}

export type OnChangeMeetingStatusCallback = (status: MeetingStatus) => unknown;
