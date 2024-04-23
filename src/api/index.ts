import { reminderSchema } from '@/types/Reminder';
import { invoke } from '@tauri-apps/api';
import { z } from 'zod';

export async function getRemindersFromDB() {
  try {
    const response = await invoke('get_reminders');
    const reminders = z.array(reminderSchema).parse(response);
    return reminders;
  } catch (error) {
    console.error(error);
    throw error;
  }
}

export async function getDueRemindersFromDB() {
  try {
    const response = await invoke('get_due_reminders');
    const reminders = z.array(reminderSchema).parse(response);
    return reminders;
  } catch (error) {
    console.error(error);
    throw error;
  }
}
