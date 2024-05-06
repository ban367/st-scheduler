<script lang="ts">
  import { parse } from "papaparse";
  import { userData } from "$lib/stores/user";
  import { calendarData } from "$lib/stores/calendar";
  import type { UserData } from "$lib/types/user";
  import type { CalendarDay } from "$lib/types/calendar";

  let fileInput: HTMLInputElement;

  async function updateStore() {
    if (fileInput.files) {
      const file = fileInput.files[0];
      if (file) {
        const text = await file.text();
        const { userData: newUserData, days } = parseData(text);

        userData.set(newUserData);
        calendarData.update((currentData) => {
          return { ...currentData, days };
        });
      } else {
        console.log("No file selected");
      }
    }
  }

  function parseData(csvText: string): { userData: UserData[]; days: { [key: string]: CalendarDay } } {
    const { data } = parse<string[]>(csvText, { header: true, skipEmptyLines: true });
    const userMap = new Map<string, number>();
    let userId = 1;
    const userData: UserData[] = [];
    const days: { [key: string]: CalendarDay } = {};

    data.forEach((row) => {
      Object.entries(row).forEach(([key, name]) => {
        if (!key.startsWith("Student") || name.trim() === "") return;
        if (!userMap.has(name)) {
          userMap.set(name, userId);
          userData.push({ id: userId, name });
          userId++;
        }
      });
    });

    data.forEach((row, index) => {
      days[String(index + 1)] = {
        userIds: Object.entries(row)
          .filter(([key, _]) => key.startsWith("Student"))
          .map(([_, name]) => userMap.get(name.trim()))
          .filter((id): id is number => id !== undefined),
        stUserIds: [],
      };
    });

    return { userData, days };
  }
</script>

<div>
  <input type="file" bind:this={fileInput} />
  <button class="w-14 rounded bg-slate-400" on:click={updateStore}> 送信 </button>
</div>
