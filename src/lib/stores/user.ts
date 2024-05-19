import { writable } from "svelte/store";
import type { UserData } from "$lib/types/user";

export const userData = writable<UserData[]>([]);
export const available_st_count = writable<number>(7);
