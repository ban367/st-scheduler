<script lang="ts">
  import { save } from "@tauri-apps/api/dialog";
  import { writeTextFile } from "@tauri-apps/api/fs";
  import Calendar from "$lib/components/Calendar.svelte";
  import Analysis from "$lib/components/Analysis.svelte";
  import { calendarDataToCSV } from "$lib/utils/convert";
  import { calendarData } from "$lib/stores/calendar";
  import { userData } from "$lib/stores/user";
  import { cButton } from "$lib/utils/constant";

  function getFormattedDateTime(): string {
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, "0");
    const day = String(now.getDate()).padStart(2, "0");
    const hours = String(now.getHours()).padStart(2, "0");
    const minutes = String(now.getMinutes()).padStart(2, "0");
    const seconds = String(now.getSeconds()).padStart(2, "0");
    return `${year}${month}${day}${hours}${minutes}${seconds}`;
  }

  async function downloadCSV() {
    const formattedDateTime = getFormattedDateTime();
    const defaultPath = `calendar_${formattedDateTime}.csv`;
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
