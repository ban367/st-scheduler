<script lang="ts">
  import Calender from "$lib/components/Calender.svelte";
  import InsertForm from "$lib/components/InsertForm.svelte";
  import Analysis from "$lib/components/Analysis.svelte";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
</script>

<h3 class="h3">調整カレンダー</h3>

<Calender />
<InsertForm />

{#if $calendarData}
  <Analysis userList={$userData} />
{/if}
