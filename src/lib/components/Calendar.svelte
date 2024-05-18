<script lang="ts">
  import { onMount } from "svelte";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";
  import { get, type Writable } from "svelte/store";
  import { currentYear, currentMonth, excludeDays } from "$lib/stores/calendar";
  import { updateUserIds } from "$lib/stores/calendar";
  import ModalUserSelect from "$lib/components/modal/UserSelect.svelte";

  const initialDate: Date = new Date();
  const unknownName = "Unknown";

  let days: { day: number | string; userIds: number[]; stUserIds: number[]; isToday: boolean; isWeekend: boolean }[] =
    [];

  function generateCalendar(month: number, year: number): void {
    let correctedMonth = month - 1; // JSの処理用に月を調整
    days = [];
    let daysInMonth: number = new Date(year, correctedMonth + 1, 0).getDate();
    let firstDayOfMonth: number = new Date(year, correctedMonth, 1).getDay();

    for (let i = 0; i < firstDayOfMonth; i++) {
      days.push({ day: "", userIds: [], stUserIds: [], isToday: false, isWeekend: false });
    }

    for (let day = 1; day <= daysInMonth; day++) {
      let date = new Date(year, correctedMonth, day);
      let isWeekend = date.getDay() === 0 || date.getDay() === 6;
      days.push({
        day: day,
        userIds:
          $calendarData &&
          $calendarData.year === year &&
          $calendarData.month === month &&
          $calendarData.days[day.toString()]
            ? $calendarData.days[day.toString()].userIds
            : [],
        stUserIds:
          $calendarData &&
          $calendarData.year === year &&
          $calendarData.month === month &&
          $calendarData.days[day.toString()]
            ? $calendarData.days[day.toString()].stUserIds
            : [],
        isToday: isToday(day, correctedMonth, year),
        isWeekend: isWeekend,
      });
    }
  }

  function isToday(day: number, month: number, year: number): boolean {
    return day === initialDate.getDate() && month === initialDate.getMonth() && year === initialDate.getFullYear();
  }

  function navigateMonth(step: number): void {
    $currentMonth += step;
    if ($currentMonth > 11) {
      $currentMonth = 0;
      $currentYear++;
    } else if ($currentMonth < 0) {
      $currentMonth = 11;
      $currentYear--;
    }
    generateCalendar($currentMonth, $currentYear);
  }

  function resetToToday(): void {
    $currentMonth = initialDate.getMonth() + 1;
    $currentYear = initialDate.getFullYear();
    generateCalendar($currentMonth, $currentYear);
  }

  function getUserName(userId: number): string {
    const user = $userData.find((user) => user.id === userId);
    return user ? user.name : unknownName;
  }

  const modalStore = getModalStore();

  const cModalBackdrop = "!bg-gray-300/80";
  function modalAddUser(day: number): void {
    const MyModalComponent: ModalComponent = {
      ref: ModalUserSelect,
      props: { title: "追加するユーザーを選択してください", userList: $userData },
    };
    const modal: ModalSettings = {
      type: "component",
      component: MyModalComponent,
      backdropClasses: cModalBackdrop,
      response: (userId: number | undefined) => {
        if (typeof userId === "number") {
          if (getUserName(userId) === unknownName) {
            return;
          }
          updateUserIds(Number(days[day].day), [...days[day].userIds, userId]);
        }
      },
    };
    modalStore.trigger(modal);
  }

  function selectDate(day: number) {
    const currentDates = get(excludeDays);
    if (!currentDates.includes(day)) {
      excludeDays.set([...currentDates, day]);
    } else {
      excludeDays.set(currentDates.filter((d) => d !== day));
    }
  }

  $: $calendarData, generateCalendar($currentMonth, $currentYear);

  onMount(() => {
    generateCalendar($currentMonth, $currentYear);
  });
</script>

<div class="p-4">
  <div class="mb-2 flex justify-between">
    <button class="rounded bg-blue-500 px-4 py-2 text-white" on:click={() => navigateMonth(-1)}>&lt; 前月</button>
    <button class="cursor-pointer rounded px-4 py-2 text-center" on:click={resetToToday}>
      {$currentYear}年 {$currentMonth}月
    </button>
    <button class="rounded bg-blue-500 px-4 py-2 text-white" on:click={() => navigateMonth(1)}>次月 &gt;</button>
  </div>
  <div class="grid grid-cols-7 gap-1">
    <div class="bg-red-200 py-2 text-center font-bold">Sun</div>
    <div class="bg-gray-200 py-2 text-center font-bold">Mon</div>
    <div class="bg-gray-200 py-2 text-center font-bold">Tue</div>
    <div class="bg-gray-200 py-2 text-center font-bold">Wed</div>
    <div class="bg-gray-200 py-2 text-center font-bold">Thu</div>
    <div class="bg-gray-200 py-2 text-center font-bold">Fri</div>
    <div class="bg-blue-200 py-2 text-center font-bold">Sat</div>
    {#each days as dayInfo, index}
      <div
        class={`p-2 text-center ${dayInfo.day ? "bg-white" : "bg-gray-100"} ${dayInfo.isToday ? "bg-yellow-200" : ""} `}
      >
        <div class="flex items-center justify-between p-2 text-center">
          <div>{dayInfo.day}</div>
          <input
            type="checkbox"
            class="cursor-pointer"
            checked={$excludeDays.includes(Number(dayInfo.day))}
            on:change={() => selectDate(Number(dayInfo.day))}
          />
        </div>
        <div class="text-xs">
          {#each dayInfo.userIds as userId}
            <div class="flex">
              <input
                type="checkbox"
                id="user-{userId}-{index}"
                checked={dayInfo.stUserIds && dayInfo.stUserIds.includes(userId)}
              />
              <label for="user-{userId}-{index}">{getUserName(userId)}</label>
            </div>
          {/each}
        </div>
        {#if dayInfo.day}
          <button class="mt-1 text-sm text-blue-500" on:click={() => modalAddUser(index)}>Add User</button>
        {/if}
      </div>
    {/each}
  </div>
</div>
