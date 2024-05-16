<script lang="ts">
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

  import { userData } from "$lib/stores/user";
  import UserList from "$lib/components/UserList.svelte";
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
</script>

<div class="ml-3">
  <div class="">
    <h4 class="h4">出席データの登録</h4>
    <div class="">
      <button class="btn btn-sm border bg-white px-6" on:click={modalInsertUser}>ユーザーの追加</button>
    </div>
  </div>

  <UserList bind:userList={$userData} />
</div>
