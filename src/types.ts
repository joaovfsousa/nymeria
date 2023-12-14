export enum MeetingStatus {
  OFF = "off",
  ON = "on",
}

export enum App {
  SLACK = "Slack",
  TEAMS = "Teams",
}

export type OnChangeMeetingStatusCallback = (status: MeetingStatus) => unknown;
