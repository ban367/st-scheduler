<script lang="ts">
  import { getModalStore } from "@skeletonlabs/skeleton";
  import type { SvelteComponent } from "svelte";
  import { cButton } from "$lib/utils/constant";

  export let parent: SvelteComponent;
  export let title: string;

  const modalStore = getModalStore();

  function onConfirm(): void {
    if (typeof $modalStore[0]?.response === "function") {
      $modalStore[0].response(true);
    }
    modalStore.close();
  }
</script>

{#if $modalStore[0]}
  <div class="w-modal relative rounded-lg bg-white p-8 shadow-xl">
    <p class="text-xl font-bold">{title}</p>
    <div class="mb-8 mt-2">
      <slot />
    </div>
    <div class="space-x-2 text-right">
      <button type="button" class={cButton} on:click={onConfirm}>はい</button>
      <button type="button" class={cButton} on:click={parent.onClose}>キャンセル</button>
    </div>
  </div>
{/if}
