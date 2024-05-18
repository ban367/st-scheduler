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

  function updateDate(value: string) {
    const [year, month] = value.split("-").map(Number);

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
          .filter(([key, _]) => key.startsWith("Student"))
          .map(([_, name]) => userMap.get(name.trim()))
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

  function onMonthInput(event: Event) {
    const target = event.target as HTMLInputElement;
    updateDate(target.value);
  }

  $: monthValue = `${$currentYear}-${String($currentMonth).padStart(2, "0")}`;
</script>

{#if $modalStore[0]}
  <div class="w-modal relative rounded-lg bg-white p-8 shadow-xl">
    <h4 class="h4 font-bold">データのアップロード</h4>
    <div class="mb-3 mt-2">
      <p>日付ごとに分割したデータをアップロードしてください</p>
    </div>
    <div class="mb-2 flex items-center">
      <label class="label flex items-center">
        <span class="pr-4">登録する年月:</span>
        <input type="month" class="!mt-0" bind:value={monthValue} on:input={onMonthInput} />
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
            <svelte:fragment slot="message">Upload a file or drag and drop</svelte:fragment>
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
        <button type="button" class={cButton} on:click={parent.onClose}> キャンセル </button>
      </div>
    </div>
  </div>
{/if}
