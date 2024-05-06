<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/user";
  import type { AnalyzeCalendarData } from "$lib/types/user";

  async function analyzeCalendarData() {
    const response = await invoke("analyze_calendar_data", { data: $calendarData });
    $calendarData = response as AnalyzeCalendarData;
  }
</script>

<button on:click={analyzeCalendarData}>Send Calendar Data</button>

<pre class="mt-2 overflow-x-auto rounded bg-gray-100 p-4">{JSON.stringify($calendarData, null, 2)}</pre>
