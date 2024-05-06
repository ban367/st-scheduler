export interface UserData {
  id: number;
  name: string;
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
  attendanceCount: { [key: number]: number };
  stCount: { [key: number]: number };
}
