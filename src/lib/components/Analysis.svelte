<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/calendar";
  import { excludeDays } from "$lib/stores/calendar";
  import { userData } from "$lib/stores/user";
  import { cButton } from "$lib/utils/constant";

  import type { AnalyzeData, UserAggregate } from "$lib/types/calendar";

  async function analyzeCalendarData() {
    const response = (await invoke("analyze_calendar_data", {
      data: $calendarData,
      excludeUserIds: getSelectExcludeUserIds(),
      excludeDates: $excludeDays,
    })) as AnalyzeData;

    $calendarData = {
      year: response.year,
      month: response.month,
      days: response.days,
    };
    updateUserListFromAggregate(response.userAggregate);
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
</script>

<div class="">
  <button class={cButton} on:click={analyzeCalendarData}>解析</button>
</div>
