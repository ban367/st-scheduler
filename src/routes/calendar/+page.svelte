<script lang="ts">
  import Calender from "$lib/components/Calender.svelte";
  import InsertForm from "$lib/components/InsertForm.svelte";
  import Analysis from "$lib/components/Analysis.svelte";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
</script>

<div>
  <Calender />
  <InsertForm />
</div>

{#if $calendarData}
  <Analysis userList={$userData} />
{/if}
