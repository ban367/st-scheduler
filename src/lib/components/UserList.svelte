<script lang="ts">
  import { Autocomplete } from "@skeletonlabs/skeleton";
  import type { AutocompleteOption } from "@skeletonlabs/skeleton";
  import type { UserData } from "$lib/types/user";

  export let userList: UserData[];

  const convertedUsers: AutocompleteOption<string>[] = userList.map((user) => ({
    value: user.id.toString(),
    label: user.name,
  }));

  let inputName = "";

  function onFlavorSelection(event: CustomEvent<AutocompleteOption<string>>): void {
    inputName = event.detail.label;
  }
</script>

<div>
  <input class="input" type="search" name="demo" bind:value={inputName} placeholder="Search..." />
  <div class="card max-h-48 w-full max-w-sm overflow-y-auto p-4" tabindex="-1">
    <Autocomplete bind:input={inputName} options={convertedUsers} on:selection={onFlavorSelection} />
  </div>
</div>
