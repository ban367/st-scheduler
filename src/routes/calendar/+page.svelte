<script lang="ts">
  import { save } from "@tauri-apps/api/dialog";
  import { writeTextFile } from "@tauri-apps/api/fs";
  import Calendar from "$lib/components/Calendar.svelte";
  import Analysis from "$lib/components/Analysis.svelte";
  import { calendarDataToCSV } from "$lib/utils/convert";
  import { calendarData } from "$lib/stores/calendar";
  import { userData } from "$lib/stores/user";
  import { cButton } from "$lib/utils/constant";

  async function downloadCSV() {
    const defaultPath = "calendar.csv";
    const csvContent = calendarDataToCSV($calendarData, $userData);

    try {
      const path = await save({ defaultPath });
      if (path) {
        await writeTextFile(path, csvContent);
        console.log(`File saved to ${path}`);
      } else {
        console.log("File save was canceled");
      }
    } catch (error) {
      console.error("Error writing file:", error);
    }
  }
</script>

<div>
  <Calendar />
</div>

{#if $calendarData}
  <div class="mb-5 mr-5 mt-2 flex justify-end">
    <div class="mx-3">
      <button class={cButton} on:click={downloadCSV}>ダウンロード</button>
    </div>
    <div>
      <Analysis />
    </div>
  </div>
{/if}
