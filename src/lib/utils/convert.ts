import type { CalendarData } from "$lib/types/calendar";
import type { UserData } from "$lib/types/user";

export function calendarDataToCSV(data: CalendarData, users: UserData[]): string {
  const userMap = new Map<number, string>();
  users.forEach((user) => {
    userMap.set(user.id, user.name);
  });

  const maxUsers = Math.max(...Object.values(data.days).map((day) => day.userIds.length));
  const headers = ["Day", ...Array.from({ length: maxUsers }, (_, i) => `Student ${i + 1}`)];
  const rows = [headers.join(",")];

  for (const [day, calendarDay] of Object.entries(data.days)) {
    const userNames = calendarDay.userIds.map((id) => {
      const name = userMap.get(id) || "";
      console.log(calendarDay.stUserIds);
      return calendarDay.stUserIds.includes(id) ? `●${name}` : name;
    });

    userNames.sort((a, b) =>
      a.startsWith("●") && !b.startsWith("●") ? -1 : !a.startsWith("●") && b.startsWith("●") ? 1 : 0,
    );

    while (userNames.length < maxUsers) {
      userNames.push("");
    }

    const row = [day, ...userNames].join(",");
    rows.push(row);
  }

  return rows.join("\n");
}
