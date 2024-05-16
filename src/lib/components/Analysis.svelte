<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/calendar";
  import { currentYear, currentMonth } from "$lib/stores/calendar";
  import { userData } from "$lib/stores/user";

  import type { AnalyzeData, UserAggregate } from "$lib/types/calendar";

  let excludeDates: number[] = [];
  let selectExcludeDates: number[] = [];

  async function analyzeCalendarData() {
    const response = (await invoke("analyze_calendar_data", {
      data: $calendarData,
      excludeUserIds: getSelectExcludeUserIds(),
      excludeDates: selectExcludeDates,
    })) as AnalyzeData;

    $calendarData = {
      year: response.year,
      month: response.month,
      days: response.days,
    };
    updateUserListFromAggregate(response.userAggregate);
  }

  function getDaysInMonth() {
    const date = new Date($currentYear, $currentMonth, 1);
    const days: number[] = [];

    while (date.getMonth() === $currentMonth) {
      days.push(date.getDate());
      date.setDate(date.getDate() + 1);
    }
    excludeDates = days;
  }

  function getSelectExcludeUserIds() {
    return $userData.filter((user) => user.isIgnore).map((user) => user.id);
  }

  function updateUserListFromAggregate(userAggregate: { [key: number]: UserAggregate }) {
    $userData = $userData.map((user) => {
      const aggregate = userAggregate[user.id];
      if (aggregate) {
        return {
          ...user,
          attendance: aggregate.attendance,
          stRewards: aggregate.stRewards,
        };
      }
      return user;
    });
  }

  $: $currentMonth, getDaysInMonth();
</script>

<div class="">
  <button class="w-12 rounded bg-slate-400" on:click={analyzeCalendarData}>解析</button>
</div>
