<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { calendarData } from "$lib/stores/user";

  async function sendCalendarData() {
    const response = await invoke("process_calendar_data", { data: $calendarData });
    console.log(response);
  }
</script>

<button on:click={sendCalendarData}>Send Calendar Data</button>

<pre class="mt-2 overflow-x-auto rounded bg-gray-100 p-4">{JSON.stringify($calendarData, null, 2)}</pre>
