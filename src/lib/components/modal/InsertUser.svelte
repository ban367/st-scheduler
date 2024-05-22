<script lang="ts">
  import { parse } from "papaparse";
  import { getModalStore, FileDropzone } from "@skeletonlabs/skeleton";
  import type { SvelteComponent } from "svelte";
  import Icon from "@iconify/svelte";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
  import { currentYear, currentMonth } from "$lib/stores/calendar";
  import { cButton } from "$lib/utils/constant";
  import type { UserData } from "$lib/types/user";
  import type { CalendarDay } from "$lib/types/calendar";

  export let parent: SvelteComponent;

  const modalStore = getModalStore();

  let fileInput: HTMLInputElement;

  function updateDate(year: number, month: number) {
    currentYear.set(year);
    currentMonth.set(month);

    calendarData.update((data) => {
      return { ...data, year: year, month: month };
    });
  }

  async function updateStore() {
    if (typeof $modalStore[0]?.response === "function") {
      if (fileInput != undefined && fileInput.files) {
        const file = fileInput.files[0];
        if (file) {
          const text = await file.text();
          const { userData: newUserData, days } = parseData(text);
          userData.set(newUserData);
          calendarData.update((currentData) => {
            return { ...currentData, days };
          });
          $modalStore[0].response(true);
        } else {
          console.log("No file selected");
        }
      }
    }
    modalStore.close();
  }

  function parseData(csvText: string): { userData: UserData[]; days: { [key: string]: CalendarDay } } {
    const { data } = parse<string[]>(csvText, { header: true, skipEmptyLines: true });
    const userMap = new Map<string, number>();
    let userId = 1;
    const userData: UserData[] = [];
    const days: { [key: string]: CalendarDay } = {};
    data.forEach((row) => {
      Object.entries(row).forEach(([key, name]) => {
        if (!key.startsWith("Student") || name.trim() === "") return;
        if (!userMap.has(name)) {
          userMap.set(name, userId);
          userData.push({ id: userId, name, attendance: 0, stRewards: 0, isIgnore: false });
          userId++;
        }
      });
    });

    data.forEach((row, index) => {
      days[String(index + 1)] = {
        userIds: Object.entries(row)
          .filter(([key]) => key.startsWith("Student"))
          .map(([, name]) => userMap.get(name.trim()))
          .filter((id): id is number => id !== undefined),
        stUserIds: [],
      };
    });
    return { userData, days };
  }

  function onChangeHandler(e: Event) {
    const target = e.target as HTMLInputElement | null;
    if (target !== null) {
      fileInput = target;
    }
  }

  function onYearInput(event: Event) {
    const target = event.target as HTMLInputElement | null;
    if (target !== null) {
      const year = parseInt(target.value);
      updateDate(year, $currentMonth);
    }
  }

  function onMonthInput(event: Event) {
    const target = event.target as HTMLInputElement | null;
    if (target !== null) {
      const month = parseInt(target.value);
      updateDate($currentYear, month);
    }
  }

  $: monthValue = String($currentMonth).padStart(2, "0");
</script>

{#if $modalStore[0]}
  <div class="w-modal relative rounded-lg bg-white p-8 shadow-xl">
    <h4 class="h4 font-bold">データのアップロード</h4>
    <div class="mb-3 mt-2">
      <p>日付ごとに分割したデータをアップロードしてください</p>
    </div>
    <div class="mb-5 flex items-center">
      <p class="pr-4">登録月:</p>
      <label class="label flex items-center">
        <div class="!mt-0 flex items-center">
          <input
            type="number"
            class="w-16 pr-2 text-center"
            bind:value={$currentYear}
            min="2000"
            max="2100"
            on:input={onYearInput}
          />
          <p class="ml-2">年</p>
        </div>
      </label>
      <label class="label ml-4 flex items-center">
        <div class="flex items-center">
          <input
            type="number"
            class="w-12 pr-2 text-center"
            bind:value={monthValue}
            min="1"
            max="12"
            on:input={onMonthInput}
          />
          <p class="ml-2">月</p>
        </div>
      </label>
    </div>
    <div class="mb-6 flex items-center justify-center">
      <div class="w-96">
        {#if fileInput != undefined}
          {#if fileInput.files && fileInput.files[0]}
            <div class="rounded border border-slate-500 bg-slate-200">
              <p class="p-5">{fileInput.files[0].name}</p>
            </div>
          {/if}
        {:else}
          <FileDropzone name="files" on:change={onChangeHandler}>
            <svelte:fragment slot="lead">
              <div class="flex h-full items-center justify-center">
                <Icon icon="mdi:file-upload" height="26" />
              </div>
            </svelte:fragment>
            <svelte:fragment slot="message">Upload a file</svelte:fragment>
            <svelte:fragment slot="meta">CSV allowed.</svelte:fragment>
          </FileDropzone>
        {/if}
      </div>
    </div>

    <div class="flex">
      <div class="flex-1 text-center">
        <button
          class={cButton}
          on:click={updateStore}
          disabled={fileInput === undefined || fileInput.files === undefined}
        >
          アップロード
        </button>
      </div>
      <div class="flex-1 text-center">
        <button type="button" class={cButton} on:click={parent.onClose}>キャンセル</button>
      </div>
    </div>
  </div>
{/if}
