import { writable } from "svelte/store";
import type { AnalyzeData } from "$lib/types/calendar";

const currentDate = new Date();

export const currentYear = writable(currentDate.getFullYear());
export const currentMonth = writable(currentDate.getMonth() + 1);

export const calendarData = writable<AnalyzeData>({
  year: currentDate.getFullYear(),
  month: currentDate.getMonth() + 1,
  days: {},
  userAggregate: {},
});

export function updateUserIds(day: number, newUserIds: number[]) {
  calendarData.update((data) => {
    if (!data.days[day]) {
      data.days[day] = {
        userIds: [],
        stUserIds: [],
      };
    }
    data.days[day].userIds = newUserIds;
    return data;
  });
}
