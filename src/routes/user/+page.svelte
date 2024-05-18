<script lang="ts">
  import { getModalStore } from "@skeletonlabs/skeleton";
  import { save } from "@tauri-apps/api/dialog";
  import { writeTextFile } from "@tauri-apps/api/fs";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

  import { userData } from "$lib/stores/user";
  import { currentYear, currentMonth, excludeDays } from "$lib/stores/calendar";
  import UserList from "$lib/components/UserList.svelte";
  import SelectCalendar from "$lib/components/SelectCalendar.svelte";
  import ModalInsertUser from "$lib/components/modal/InsertUser.svelte";

  import { calendarData } from "$lib/stores/calendar";
  import { calendarDataToCSV } from "$lib/utils/convert";

  const modalStore = getModalStore();

  let selectDay = excludeDays;

  async function downloadCSV() {
    const defaultPath = "calendar.csv";
    const csvContent = calendarDataToCSV($calendarData, $userData);

    try {
      const path = await save({ defaultPath });
      if (path) {
        await writeTextFile(path, csvContent);
        console.log(`File saved to ${path}`);
      } else {
        console.log("File save was canceled");
      }
    } catch (error) {
      console.error("Error writing file:", error);
    }
  }

  function modalInsertUser(): void {
    const MyModalComponent: ModalComponent = {
      ref: ModalInsertUser,
    };
    const modal: ModalSettings = {
      type: "component",
      component: MyModalComponent,
      backdropClasses: "!bg-gray-300/80",
      response: (isConfirm: boolean) => {
        if (isConfirm) {
          console.log("Confirmed");
        }
      },
    };
    modalStore.trigger(modal);
  }
</script>

<div class="ml-3">
  <div class="">
    <h4 class="h4">出席データの登録</h4>
    <div class="">
      <button class="btn btn-sm border bg-white px-6" on:click={modalInsertUser}>ユーザーの追加</button>
    </div>
  </div>

  <button class="btn" on:click={downloadCSV}>CSVをダウンロード</button>

  <div class="grid grid-cols-5 gap-4">
    <div class="col-span-2">
      <UserList bind:userList={$userData} />
    </div>
    <div class="col-span-2">
      <SelectCalendar year={$currentYear} month={$currentMonth - 1} bind:selectDays={selectDay} />
    </div>
  </div>
</div>
