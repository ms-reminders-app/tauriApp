import { ReminderInsert } from "@/types/Reminder";
import { atom } from "jotai";

export const remindersState = atom<ReminderInsert[]>([]);