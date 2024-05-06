import { writable } from "svelte/store";
import type { AnalyzeData } from "$lib/types/calendar";

const currentDate = new Date();
const currentYear = currentDate.getFullYear();
const currentMonth = currentDate.getMonth() + 1;

export const calendarData = writable<AnalyzeData>({
  year: currentYear,
  month: currentMonth,
  days: {},
  userAggregate: {},
});
