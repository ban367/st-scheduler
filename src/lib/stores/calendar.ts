import { writable } from "svelte/store";
import { userData } from "$lib/stores/user";
import type { CalendarData, CalendarDay } from "$lib/types/calendar";

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

function getDaysInMonth(year: number, month: number): number {
  return new Date(year, month, 0).getDate();
}

function updateDays(year: number, month: number) {
  const daysInMonth = getDaysInMonth(year, month);
  const newDays: { [key: number]: CalendarDay } = {};
  for (let day = 1; day <= daysInMonth; day++) {
    newDays[day] = {
      userIds: [],
      stUserIds: [],
    };
  }
  return newDays;
}

export function initNextMonth() {
  const newDate = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1);

  const newYear = newDate.getFullYear();
  const newMonth = newDate.getMonth() + 1;

  currentYear.set(newYear);
  currentMonth.set(newMonth);

  const newDays = updateDays(newYear, newMonth);

  calendarData.set({
    year: newYear,
    month: newMonth,
    days: newDays,
  });
}

currentYear.subscribe((year) => {
  let month;
  currentMonth.subscribe((m) => {
    month = m;
    const newDays = updateDays(year, month);
    calendarData.update((data) => {
      data.days = newDays;
      return data;
    });
    userData.set([]);
  });
});

currentMonth.subscribe((month) => {
  let year;
  currentYear.subscribe((y) => {
    year = y;
    const newDays = updateDays(year, month);
    calendarData.update((data) => {
      data.days = newDays;
      return data;
    });
    userData.set([]);
  });
});
