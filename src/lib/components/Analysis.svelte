<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/calendar";
  import { currentYear, currentMonth } from "$lib/stores/calendar";
  import type { AnalyzeData } from "$lib/types/calendar";
  import type { UserData } from "$lib/types/user";

  export let userList: UserData[];

  let excludeUserIds: number[] = [];
  let selectExcludeUserIds: number[] = [];
  let excludeDates: number[] = [];
  let selectExcludeDates: number[] = [];

  function handleSelectExcludeDates(event: Event) {
    const select = event.target as HTMLSelectElement;
    selectExcludeDates = Array.from(select.selectedOptions).map((opt) => parseInt(opt.value));
  }

  function handleSelectExcludeUserIds(event: Event) {
    const select = event.target as HTMLSelectElement;
    selectExcludeUserIds = Array.from(select.selectedOptions).map((opt) => parseInt(opt.value));
  }

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
  <select class="select" multiple on:change={handleSelectExcludeDates}>
    {#each excludeDates as day}
      <option value={day}>{day}</option>
    {/each}
  </select>
  <select class="select" multiple on:change={handleSelectExcludeUserIds}>
    {#each userList as user}
      <option value={user.id}>{user.name}</option>
    {/each}
  </select>
  <button class="w-12 rounded bg-slate-400" on:click={analyzeCalendarData}>解析</button>
</div>

<!-- デバッグ用 -->
<!-- <pre class="mt-2 overflow-x-auto rounded bg-gray-100 p-4">{JSON.stringify($calendarData, null, 2)}</pre> -->
