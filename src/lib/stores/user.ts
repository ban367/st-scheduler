import { writable } from "svelte/store";
import type { UserData } from "$lib/types/user";

export const userData = writable<UserData[]>([]);
