<script lang="ts">
  import { get, type Writable } from "svelte/store";

  export let year: number;
  export let month: number;
  export let selectDays: Writable<number[]>;

  const getDaysInMonth = (year: number, month: number) => {
    const date = new Date(year, month, 1);
    const days = [];
    while (date.getMonth() === month) {
      days.push(date.getDate());
      date.setDate(date.getDate() + 1);
    }
    return days;
  };

  const daysInMonth = getDaysInMonth(year, month);
  const firstDayOfMonth = new Date(year, month, 1).getDay();

  function selectDate(day: number) {
    const currentDates = get(selectDays);
    if (!currentDates.includes(day)) {
      selectDays.set([...currentDates, day]);
    } else {
      selectDays.set(currentDates.filter((d) => d !== day));
    }
  }
</script>

<div class="grid grid-cols-7 gap-1">
  {#each ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] as day}
    <div class="text-center font-bold">{day}</div>
  {/each}
  {#each Array(firstDayOfMonth).fill(null) as _}
    <div></div>
  {/each}
  {#each daysInMonth as day}
    <button
      type="button"
      class="cursor-pointer p-2 text-center {$selectDays.includes(day) ? 'bg-blue-700 text-white' : ''}"
      on:click={() => selectDate(day)}
      on:keydown={(e) => e.key === "Enter" && selectDate(day)}
      aria-pressed={$selectDays.includes(day)}
    >
      {day}
    </button>
  {/each}
</div>
