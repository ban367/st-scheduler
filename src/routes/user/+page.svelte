<script lang="ts">
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

  import { userData } from "$lib/stores/user";
  import { currentYear, currentMonth, excludeDays } from "$lib/stores/calendar";
  import UserList from "$lib/components/UserList.svelte";
  import SelectCalendar from "$lib/components/SelectCalendar.svelte";
  import ModalInsertUser from "$lib/components/modal/InsertUser.svelte";

  const modalStore = getModalStore();

  const cModalBackdrop = "!bg-gray-300/80";
  function modalInsertUser(): void {
    const MyModalComponent: ModalComponent = {
      ref: ModalInsertUser,
    };
    const modal: ModalSettings = {
      type: "component",
      component: MyModalComponent,
      backdropClasses: cModalBackdrop,
      response: (isConfirm: boolean) => {
        if (isConfirm) {
          console.log("Confirmed");
        }
      },
    };
    modalStore.trigger(modal);
  }

  let selectDay = excludeDays;
</script>

<div class="ml-3">
  <div class="">
    <h4 class="h4">出席データの登録</h4>
    <div class="">
      <button class="btn btn-sm border bg-white px-6" on:click={modalInsertUser}>ユーザーの追加</button>
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4">
    <div class="col-span-2">
      <UserList bind:userList={$userData} />
    </div>
    <div class="col-span-2">
      <SelectCalendar year={$currentYear} month={$currentMonth - 1} bind:selectDays={selectDay} />
    </div>
  </div>
</div>
