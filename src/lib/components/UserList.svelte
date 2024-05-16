<script lang="ts">
  import type { UserData } from "$lib/types/user";

  export let userList: UserData[];

  function toggleIgnore(index: number) {
    userList = [
      ...userList.slice(0, index),
      { ...userList[index], isIgnore: !userList[index].isIgnore },
      ...userList.slice(index + 1),
    ];
  }
</script>

<div class="table-container">
  <table class="table table-hover">
    <thead>
      <tr>
        <th>名前</th>
        <th>出席数</th>
        <th>ST割当数</th>
        <th>ST対象</th>
      </tr>
    </thead>
    <tbody>
      {#each userList as user, index}
        <tr>
          <td>{user.name}</td>
          <td>{user.attendance}</td>
          <td>{user.stRewards}</td>
          <td>
            <input type="checkbox" checked={!user.isIgnore} on:change={() => toggleIgnore(index)} />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
