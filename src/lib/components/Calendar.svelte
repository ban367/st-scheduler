<script lang="ts">
  import { onMount } from "svelte";
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

  import { userData } from "$lib/stores/user";
  import { calendarData, currentYear, currentMonth, excludeDays, updateUserIds } from "$lib/stores/calendar";
  import ModalUserSelect from "$lib/components/modal/UserSelect.svelte";
  import ModalSubmit from "$lib/components/modal/Submit.svelte";
  import Icon from "@iconify/svelte";

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
          $calendarData.year === year && $calendarData.month === month && $calendarData.days[day.toString()]
            ? $calendarData.days[day.toString()].userIds
            : [],
        stUserIds:
          $calendarData.year === year && $calendarData.month === month && $calendarData.days[day.toString()]
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
    let newMonth = $currentMonth + step;
    let newYear = $currentYear;

    if (newMonth > 12) {
      newMonth = 1;
      newYear++;
    } else if (newMonth < 1) {
      newMonth = 12;
      newYear--;
    }

    currentYear.set(newYear);
    currentMonth.set(newMonth);
  }

  function toggleStUserId(day: number, userId: number): void {
    calendarData.update((data) => {
      if (data.days[day]) {
        const index = data.days[day].stUserIds.indexOf(userId);
        if (index > -1) {
          data.days[day].stUserIds.splice(index, 1);
        } else {
          data.days[day].stUserIds.push(userId);
        }
      }
      return data;
    });
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
        if (typeof userId === "number" && getUserName(userId) !== unknownName) {
          updateUserIds(Number(days[day].day), [...days[day].userIds, userId]);
        }
      },
    };
    modalStore.trigger(modal);
  }
  function modalChangeMonth(step: number): void {
    if ($userData.length === 0) {
      navigateMonth(step);
      return;
    }
    const MyModalComponent: ModalComponent = {
      ref: ModalSubmit,
      props: { title: "月を変更してもよろしいですか？" },
      slot: "(変更すると登録情報は初期化されます)",
    };
    const modal: ModalSettings = {
      type: "component",
      component: MyModalComponent,
      backdropClasses: cModalBackdrop,
      response: (isConfirm: boolean) => {
        if (isConfirm) {
          navigateMonth(step);
        }
      },
    };
    modalStore.trigger(modal);
  }

  function selectDate(day: number) {
    excludeDays.update((currentDates) => {
      if (!currentDates.includes(day)) {
        return [...currentDates, day];
      } else {
        return currentDates.filter((d) => d !== day);
      }
    });
  }

  $: $calendarData, generateCalendar($currentMonth, $currentYear);

  onMount(() => {
    generateCalendar($currentMonth, $currentYear);
  });
</script>

<div class="p-4">
  <div class="mb-2 flex justify-between">
    <button class="flex items-center rounded bg-blue-500 px-4 py-2 text-white" on:click={() => modalChangeMonth(-1)}>
      <Icon icon="mdi:chevron-left" class="h-5 w-5" />
      <span>前月</span>
    </button>
    <div class="flex items-center">
      <p>{$currentYear}年 {$currentMonth}月</p>
    </div>
    <button class="flex items-center rounded bg-blue-500 px-4 py-2 text-white" on:click={() => modalChangeMonth(1)}>
      <span>次月</span>
      <Icon icon="mdi:chevron-right" class="h-5 w-5" />
    </button>
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
          {#if dayInfo.day}
            <div>{dayInfo.day}</div>
            <div class="flex items-center">
              <input
                type="checkbox"
                id="day-{dayInfo.day}"
                class="mr-1 h-4 w-4 cursor-pointer accent-sky-300"
                checked={!$excludeDays.includes(Number(dayInfo.day))}
                on:change={() => selectDate(Number(dayInfo.day))}
              />
              <label for="day-{dayInfo.day}" class="flex-1 text-left">ST割当</label>
            </div>
          {/if}
        </div>
        <div class="text-xs">
          {#each dayInfo.userIds as userId}
            <div class="flex">
              <div class="flex items-center">
                <input
                  type="checkbox"
                  id="user-{userId}-{index}"
                  class="mr-1 h-4 w-4 cursor-pointer accent-emerald-300"
                  checked={dayInfo.stUserIds && dayInfo.stUserIds.includes(userId)}
                  on:change={() => toggleStUserId(Number(dayInfo.day), userId)}
                />
                <label for="user-{userId}-{index}">{getUserName(userId)}</label>
              </div>
            </div>
          {/each}
        </div>
        {#if dayInfo.day}
          <button class="mt-1 text-sm text-blue-500" on:click={() => modalAddUser(index)}>ユーザー追加</button>
        {/if}
      </div>
    {/each}
  </div>
</div>
