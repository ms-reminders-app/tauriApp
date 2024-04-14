// import { remindersState } from "@/state/remindersState";
import { ReminderInsert } from "@/types/Reminder";
// import { useAtom } from "jotai";
import { useEffect } from "react";
import { DataTable } from "../remindersTable/data-table";
import { useQuery, /* useQueryClient */ } from "@tanstack/react-query";
import { getRemindersFromDB } from "@/api";

function HomePage() {
  // const [reminders, setReminders] = useAtom(remindersState);
  // const queryClient = useQueryClient();
  const reminders = useQuery({ queryKey: ['reminders'], queryFn: getRemindersFromDB })

  return (
    <div className="p-2">
      <h1 className="text-center text-2xl">Reminders</h1>
      {reminders.isLoading ? <p>Loading...</p> :
        <DataTable data={reminders.data ?? []} />
      }
    </div>
  );
}

export default HomePage;