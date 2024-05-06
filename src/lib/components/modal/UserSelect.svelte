<script lang="ts">
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { SvelteComponent } from "svelte";
  import type { UserData } from "$lib/types/user";
  import type { InputEventDetail } from "$lib/types/form";

  import UserSearch from "$lib/components/UserSearch.svelte";

  export let parent: SvelteComponent;
  export let title: string;
  export let userList: UserData[];

  const modalStore = getModalStore();

  let selectUserId = 0;

  function onConfirm(): void {
    if (typeof $modalStore[0]?.response === "function") {
      $modalStore[0].response(selectUserId);
    }
    modalStore.close();
  }

  function findUserIdByName(users: UserData[], searchName: string): number | undefined {
    const user = users.find((user) => user.name === searchName);
    return user?.id;
  }

  function handleInput(event: CustomEvent<InputEventDetail>) {
    const userId = findUserIdByName(userList, event.detail.value);
    if (userId) {
      selectUserId = userId;
    }
  }

  const cButton = "btn bg-white border border-gray-400 px-12 focus:!outline-none";
</script>

{#if $modalStore[0]}
  <div class="w-modal relative rounded-lg bg-white p-8 shadow-xl">
    <p class="text-xl font-bold">{title}</p>
    <div class="mb-8 mt-2">
      <UserSearch {userList} on:input={handleInput} />
    </div>
    <div class="space-x-2 text-right">
      <button type="button" class={cButton} on:click={onConfirm}>はい</button>
      <button type="button" class={cButton} on:click={parent.onClose}>キャンセル</button>
    </div>
  </div>
{/if}
