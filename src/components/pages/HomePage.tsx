import { remindersState } from "@/state/remindersState";
import { ReminderInsert } from "@/types/Reminder";
import { useAtom } from "jotai";
import { useEffect } from "react";
import { DataTable } from "../remindersTable/data-table";

function HomePage() {
  const [reminders, setReminders] = useAtom(remindersState);

  // fill with test data
  useEffect(() => {
    const testReminders: ReminderInsert[] = [
      {
        title: "Test 1",
        description: "This is test 1",
        completed: false,
        datetime: Date.now(),
        reminder: 0
      },
      {
        title: "Test 2",
        description: "This is test 2",
        completed: false,
        datetime: Date.now(),
        reminder: 0
      },
      {
        title: "Test 3",
        description: "This is test 3",
        completed: false,
        datetime: Date.now(),
        reminder: 0
      }
    ];

    setReminders(testReminders);
  }, []) // TODO: FILL FROM LOCALSTORAGE OR RUST

  return (
    <div className="p-2">
      <h1 className="text-center text-2xl">Reminders</h1>
      <DataTable data={reminders} />
    </div>
  );
}

export default HomePage;