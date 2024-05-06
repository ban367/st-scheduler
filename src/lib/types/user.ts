export interface UserData {
  id: number;
  name: string;
}

interface UserAggregate {
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

export interface AnalyzeCalendarData {
  year: number;
  month: number;
  days: { [key: string]: CalendarDay };
  userAggregate: { [key: number]: UserAggregate };
}
