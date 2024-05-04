<script lang="ts">
  import "../app.postcss";
  import { AppBar } from "@skeletonlabs/skeleton";
  import { Toast, Modal, initializeStores } from "@skeletonlabs/skeleton";
  import Icon from "@iconify/svelte";

  initializeStores();
</script>

<Modal />
<Toast position="tr" />

<svelte:head>
  <title>Schedule</title>
</svelte:head>

<div class="flex min-h-screen flex-col">
  <div class="border-b">
    <AppBar class="!p-3">
      <div class="flex items-center">
        <a href="/" class="flex items-center">
          <Icon icon="mdi-light:home" height="20" />
          <div class="text-lg">Top</div>
        </a>
      </div>
    </AppBar>
  </div>

  <div class="mx-2 flex-1 overflow-auto">
    <slot />
  </div>

  <div class="w-full border-t bg-gray-500 px-2 text-center text-sm text-white">
    <p>フッター</p>
  </div>
</div>
