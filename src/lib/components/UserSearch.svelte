<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Autocomplete } from "@skeletonlabs/skeleton";
  import type { AutocompleteOption } from "@skeletonlabs/skeleton";
  import type { UserData } from "$lib/types/user";
  import type { InputEventDetail } from "$lib/types/form";

  export let userList: UserData[];

  interface Events {
    input: InputEventDetail;
  }

  const dispatch = createEventDispatcher<Events>();
  const convertedUsers: AutocompleteOption<string>[] = userList.map((user) => ({
    value: user.id.toString(),
    label: user.name,
  }));

  let inputName = "";

  function onFlavorSelection(event: CustomEvent<AutocompleteOption<string>>): void {
    inputName = event.detail.label;
  }

  function handleInput(event: Event) {
    const inputElement = event.target as HTMLInputElement;
    inputName = inputElement.value;
  }

  $: dispatch("input", { value: inputName });
</script>

<div>
  <input
    class="input"
    type="search"
    name="demo"
    bind:value={inputName}
    on:input={handleInput}
    placeholder="Search..."
  />
  <div class="card max-h-48 w-full max-w-sm overflow-y-auto p-4" tabindex="-1">
    <Autocomplete bind:input={inputName} options={convertedUsers} on:selection={onFlavorSelection} />
  </div>
</div>
