<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/calendar";
  import { currentYear, currentMonth } from "$lib/stores/calendar";
  import type { AnalyzeData } from "$lib/types/calendar";

  let selectExcludeUserIds: number[] = [];
  let excludeDates: number[] = [];
  let selectExcludeDates: number[] = [];

  async function analyzeCalendarData() {
    const response = await invoke("analyze_calendar_data", {
      data: $calendarData,
      excludeUserIds: selectExcludeUserIds,
      excludeDates: selectExcludeDates,
    });
    $calendarData = response as AnalyzeData;
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

  $: $currentMonth, getDaysInMonth();
</script>

<div class="">
  <button class="w-12 rounded bg-slate-400" on:click={analyzeCalendarData}>解析</button>
</div>

<!-- デバッグ用 -->
<!-- <pre class="mt-2 overflow-x-auto rounded bg-gray-100 p-4">{JSON.stringify($calendarData, null, 2)}</pre> -->
