<script lang="ts">
  import Icon from "@iconify/svelte";
  export let message = "調整中";
</script>

<div class="relative flex min-h-screen items-center justify-center">
  <div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform text-center">
    <div class="flex items-center">
      <Icon icon="mdi:scheduled-maintenance" height="30" />
      <p class="ml-2 text-2xl">{message}</p>
    </div>
  </div>
</div>
