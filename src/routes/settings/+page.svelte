<script lang="ts">
  import { available_st_count } from "$lib/stores/user";
</script>

<div class="ml-4 mt-4">
  <h4 class="h4 mb-2">設定</h4>
  <div class="flex items-center">
    <label for="countInput" class="mr-2">1日のST割当数:</label>
    <input type="number" bind:value={$available_st_count} class="w-24 rounded border p-2" />
  </div>
</div>
