import { ColumnDef } from "@tanstack/react-table"
import { ReminderInsert } from "@/types/Reminder";
import { Checkbox } from "../shadcn/ui/checkbox";
import { remindersState } from "@/state/remindersState";
import { useAtom } from "jotai";

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.

export const columns: ColumnDef<ReminderInsert>[] = [
    {
        accessorKey: "title",
        header: "Title",
    },
    {
        accessorKey: "description",
        header: "Description",
    },
    {
        accessorKey: "completed",
        header: "Completed",
        cell: ({ row }) => {
            const [reminders, setReminders] = useAtom(remindersState);

            function handleCheckboxChange() {
                const newReminders = reminders.map((reminder) => {
                    if (reminder.title === row.getValue("title")) {
                        return {
                            ...reminder,
                            completed: !reminder.completed
                        }
                    } else {
                        return reminder;
                    }
                });

                setReminders(newReminders);
            }

            return (
                <Checkbox checked={row.getValue("completed")} onCheckedChange={handleCheckboxChange}></Checkbox>
            );
        }
    },
    {
        accessorKey: "datetime",
        header: "Date",
        cell: ({ row }) => {
            return new Date(parseInt(row.getValue("datetime"))).toLocaleString().split(":").slice(0, 2).join(":");
        }
    },
    {
        accessorKey: "reminder",
        header: "Reminder",
    },
]
