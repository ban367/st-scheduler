<script lang="ts">
  import { onMount } from "svelte";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

  import { currentYear, currentMonth } from "$lib/stores/calendar";
  import { updateUserIds } from "$lib/stores/calendar";
  import ModalUserSelect from "$lib/components/modal/UserSelect.svelte";

  const initialDate: Date = new Date();
  const unknownName = "Unknown";
  // let today: Date = new Date();
  // let currentMonth: number = today.getMonth();
  // let $currentYear: number = today.getFullYear();

  let days: { day: number | string; userIds: number[]; stUserIds: number[]; isToday: boolean; isWeekend: boolean }[] =
    [];

  function generateCalendar(month: number, year: number): void {
    days = [];
    let daysInMonth: number = new Date(year, month + 1, 0).getDate();
    let firstDayOfMonth: number = new Date(year, month, 1).getDay();

    for (let i = 0; i < firstDayOfMonth; i++) {
      days.push({ day: "", userIds: [], stUserIds: [], isToday: false, isWeekend: false });
    }

    for (let day = 1; day <= daysInMonth; day++) {
      let date = new Date(year, month, day);
      let isWeekend = date.getDay() === 0 || date.getDay() === 6;
      days.push({
        day: day,
        userIds:
          $calendarData &&
          $calendarData.year === year &&
          $calendarData.month === month + 1 &&
          $calendarData.days[day.toString()]
            ? $calendarData.days[day.toString()].userIds
            : [],
        stUserIds:
          $calendarData &&
          $calendarData.year === year &&
          $calendarData.month === month + 1 &&
          $calendarData.days[day.toString()]
            ? $calendarData.days[day.toString()].stUserIds
            : [],
        isToday: isToday(day, month, year),
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
    $currentMonth = initialDate.getMonth();
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

  $: $calendarData, generateCalendar($currentMonth, $currentYear);

  onMount(() => {
    generateCalendar($currentMonth, $currentYear);
  });
</script>

<div class="p-4">
  <div class="mb-2 flex justify-between">
    <button class="rounded bg-blue-500 px-4 py-2 text-white" on:click={() => navigateMonth(-1)}>&lt; 前月</button>
    <button class="cursor-pointer rounded px-4 py-2 text-center" on:click={resetToToday}>
      {$currentYear}年 {$currentMonth + 1}月
    </button>
    <button class="rounded bg-blue-500 px-4 py-2 text-white" on:click={() => navigateMonth(1)}>次月 &gt;</button>
  </div>
  <div class="grid grid-cols-7 gap-1">
    <div class="bg-red-200 p-2 text-center">日</div>
    <div class="bg-gray-200 p-2 text-center">月</div>
    <div class="bg-gray-200 p-2 text-center">火</div>
    <div class="bg-gray-200 p-2 text-center">水</div>
    <div class="bg-gray-200 p-2 text-center">木</div>
    <div class="bg-gray-200 p-2 text-center">金</div>
    <div class="bg-blue-200 p-2 text-center">土</div>
    {#each days as dayInfo, index}
      <div
        class={`p-2 text-center ${dayInfo.day ? "bg-white" : "bg-gray-100"} ${dayInfo.isToday ? "bg-yellow-200" : ""} `}
      >
        <div>{dayInfo.day}</div>
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
