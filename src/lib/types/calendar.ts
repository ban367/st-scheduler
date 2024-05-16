export interface UserAggregate {
  attendance: number;
  stRewards: number;
}

export interface CalendarDay {
  userIds: number[];
  stUserIds: number[];
}

export interface CalendarData {
  year: number;
  month: number;
  days: { [key: string]: CalendarDay };
}

export interface AnalyzeData extends CalendarData {
  userAggregate: { [key: number]: UserAggregate };
}
