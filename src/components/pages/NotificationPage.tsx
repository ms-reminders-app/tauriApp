import { useEffect } from "react";
import { DataTable } from "../remindersTable/data-table";
import { useQuery, /* useQueryClient */ } from "@tanstack/react-query";
import { getDueRemindersFromDB } from "@/api";

export function NotificationPage() {
  // const [reminders, setReminders] = useAtom(remindersState);
  // const queryClient = useQueryClient();
  const reminders = useQuery({ queryKey: ['dueReminders'], queryFn: getDueRemindersFromDB })

  useEffect(() => {
    if (import.meta.env.MODE === 'development') {
      console.log("components/pages/NotificationPage.tsx: Reminders changed to ", reminders.data); // log reminders every time it is changed, while in development
    }
  }, [reminders.data]);

  return (
    <div className="p-2">
      <h1 className="text-center text-2xl">Reminders</h1>
      {reminders.isLoading ? <p>Loading...</p> :
        <DataTable data={reminders.data ?? []} />
      }
    </div>
  );
}
