<script lang="ts">
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { SvelteComponent } from "svelte";
  import type { UserData } from "$lib/types/user";

  import UserList from "$lib/components/UserList.svelte";

  export let parent: SvelteComponent;
  export let title: string;
  export let userList: UserData[];

  const modalStore = getModalStore();

  // TODO: UserListから結果を受け取って格納する
  let selectUserId = 1;

  function onConfirm(): void {
    if (typeof $modalStore[0]?.response === "function") {
      $modalStore[0].response(selectUserId);
    }
    modalStore.close();
  }

  const cButton = "btn bg-white border border-gray-400 px-12 focus:!outline-none";
</script>

{#if $modalStore[0]}
  <div class="w-modal relative rounded-lg bg-white p-8 shadow-xl">
    <p class="text-xl font-bold">{title}</p>
    <div class="mb-8 mt-2">
      <UserList {userList} />
    </div>
    <div class="space-x-2 text-right">
      <button type="button" class={cButton} on:click={onConfirm}>はい</button>
      <button type="button" class={cButton} on:click={parent.onClose}>キャンセル</button>
    </div>
  </div>
{/if}
