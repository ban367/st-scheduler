import { writable } from "svelte/store";
import type { CalendarData } from "$lib/types/calendar";

const currentDate = new Date();

export const currentYear = writable(currentDate.getFullYear());
export const currentMonth = writable(currentDate.getMonth() + 1);

export const excludeDays = writable<number[]>([]);

export const calendarData = writable<CalendarData>({
  year: currentDate.getFullYear(),
  month: currentDate.getMonth() + 1,
  days: {},
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

export function initNextMonth() {
  const newDate = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1);

  const newYear = newDate.getFullYear();
  const newMonth = newDate.getMonth() + 1;

  currentYear.set(newYear);
  currentMonth.set(newMonth);

  calendarData.set({
    year: newYear,
    month: newMonth,
    days: {},
  });
}
