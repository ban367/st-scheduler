<script lang="ts">
  import { onMount } from "svelte";
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

  import { userData } from "$lib/stores/user";
  import UserList from "$lib/components/UserList.svelte";
  import ModalInsertUser from "$lib/components/modal/InsertUser.svelte";
  import { cButton } from "$lib/utils/constant";

  const modalStore = getModalStore();

  function scrollToTop() {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

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
          scrollToTop();
        }
      },
    };
    modalStore.trigger(modal);
  }

  onMount(() => {
    if ($userData.length === 0) {
      modalInsertUser();
    }
  });
</script>

<div class="ml-4 mt-4">
  {#if $userData.length === 0}
    <p>データが登録されていません</p>
  {:else}
    <div class="w-[600px]">
      <div class="">
        <UserList bind:userList={$userData} />
      </div>
    </div>
    <div class="mb-5 mr-5 mt-2 flex justify-end">
      <div class="mx-3">
        <button class={cButton} on:click={modalInsertUser}>データの再登録</button>
      </div>
    </div>
  {/if}
</div>
