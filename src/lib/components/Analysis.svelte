<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/calendar";
  import type { AnalyzeData } from "$lib/types/calendar";

  const excludeUserIds = [1, 2];
  const excludeDates = [7, 15];

  async function analyzeCalendarData() {
    const response = await invoke("analyze_calendar_data", {
      data: $calendarData,
      excludeUserIds: excludeUserIds,
      excludeDates: excludeDates,
    });
    console.log(response);
    $calendarData = response as AnalyzeData;
  }
</script>

<button on:click={analyzeCalendarData}>Send Calendar Data</button>

<pre class="mt-2 overflow-x-auto rounded bg-gray-100 p-4">{JSON.stringify($calendarData, null, 2)}</pre>
