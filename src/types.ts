export enum MeetingStatus {
  OFF,
  ON,
}

export type OnChangeMeetingStatusCallback = (status: MeetingStatus) => unknown;
