import { useEffect } from "react";
import "./App.css";
import { ReminderInsert } from "./types/Reminder";
import { DataTable } from "./components/remindersTable/data-table";
import { remindersState } from "./state/remindersState";
import { useAtom } from "jotai";

function App() {
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

export default App;
