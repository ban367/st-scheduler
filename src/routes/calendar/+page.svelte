<script lang="ts">
  import Calendar from "$lib/components/Calendar.svelte";
  import Analysis from "$lib/components/Analysis.svelte";
  import { calendarData } from "$lib/stores/calendar";
</script>

<div>
  <Calendar />
</div>

{#if $calendarData}
  <div class="mb-5 mr-5 mt-2 flex justify-end">
    <div class="mx-3">
      <button class="rounded-full border bg-slate-300 px-5 py-2">ダウンロード</button>
    </div>
    <div>
      <Analysis />
    </div>
  </div>
{/if}
