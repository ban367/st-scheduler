<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "@iconify/svelte";
  import type { UserData } from "$lib/types/user";

  export let userList: UserData[];

  let sortColumn: keyof UserData | "" = "";
  let sortDirection: "asc" | "desc" = "desc";

  function toggleIgnore(index: number) {
    userList = [
      ...userList.slice(0, index),
      { ...userList[index], isIgnore: !userList[index].isIgnore },
      ...userList.slice(index + 1),
    ];
  }

  function sort(column: keyof UserData) {
    if (sortColumn === column) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
    } else {
      sortColumn = column;
      sortDirection = "asc";
    }

    userList = [...userList].sort((a, b) => {
      if (a[column] < b[column]) {
        return sortDirection === "asc" ? -1 : 1;
      }
      if (a[column] > b[column]) {
        return sortDirection === "asc" ? 1 : -1;
      }
      return 0;
    });
  }

  onMount(() => {
    sort("name");
  });
</script>

<div class="table-container">
  <table class="table table-hover bg-white">
    <thead class="">
      <tr class="bg-gray-200">
        <th class="cursor-pointer" on:click={() => sort("name")}>
          <div class="flex w-28 items-center justify-center">
            名前
            {#if sortColumn === "name"}
              <span>
                <Icon icon={sortDirection === "asc" ? "mdi:arrow-down" : "mdi:arrow-up"}></Icon>
              </span>
            {/if}
          </div>
        </th>
        <th class="cursor-pointer" on:click={() => sort("attendance")}>
          <div class="flex items-center justify-center">
            出席数
            {#if sortColumn === "attendance"}
              <span>
                <Icon icon={sortDirection === "asc" ? "mdi:arrow-down" : "mdi:arrow-up"}></Icon>
              </span>
            {/if}
          </div>
        </th>
        <th class="cursor-pointer" on:click={() => sort("stRewards")}>
          <div class="flex items-center justify-center">
            ST割当数
            {#if sortColumn === "stRewards"}
              <span>
                <Icon icon={sortDirection === "asc" ? "mdi:arrow-down" : "mdi:arrow-up"}></Icon>
              </span>
            {/if}
          </div>
        </th>
        <th class="text-center">ST対象</th>
      </tr>
    </thead>
    <tbody>
      {#each userList as user, index}
        <tr>
          <td class="">{user.name}</td>
          <td class="text-center">{user.attendance}</td>
          <td class="text-center">{user.stRewards}</td>
          <td class="text-center">
            <input
              type="checkbox"
              class="mr-1 h-4 w-4 cursor-pointer accent-sky-300"
              checked={!user.isIgnore}
              on:change={() => toggleIgnore(index)}
            />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
