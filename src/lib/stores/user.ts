import { writable } from "svelte/store";
import type { UserData, CalendarData } from "$lib/types/user";

const currentDate = new Date();
const currentYear = currentDate.getFullYear();
const currentMonth = currentDate.getMonth() + 1;

export const userData = writable<UserData[]>([]);
export const calendarData = writable<CalendarData>({
  year: currentYear,
  month: currentMonth,
  days: {},
});
