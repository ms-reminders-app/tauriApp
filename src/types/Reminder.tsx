import { z } from "zod";

export const reminderSchema = z.object({
    id: z.number(),
    title: z.string(),
    description: z.string(),
    completed: z.boolean(),
    datetime: z.number(),
    reminder: z.number(),
})

export type Reminder = z.infer<typeof reminderSchema>;

export const reminderInsertSchema = reminderSchema.omit({ id: true });

export type ReminderInsert = z.infer<typeof reminderInsertSchema>;
