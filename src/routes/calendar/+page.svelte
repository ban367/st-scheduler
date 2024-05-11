<script lang="ts">
  import Calender from "$lib/components/Calender.svelte";
  import Analysis from "$lib/components/Analysis.svelte";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
</script>

<div>
  <Calender />
</div>

{#if $calendarData}
  <Analysis />
{/if}
