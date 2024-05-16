<script lang="ts">
  import { currentYear, currentMonth } from "$lib/stores/calendar";

  let excludeDates: number[] = [];
  let selectExcludeDates: number[] = [];

  function handleSelectExcludeDates(event: Event) {
    const select = event.target as HTMLSelectElement;
    selectExcludeDates = Array.from(select.selectedOptions).map((opt) => parseInt(opt.value));
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
</div>
