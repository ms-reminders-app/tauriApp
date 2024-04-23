import "./App.css";
import {
  QueryClient,
  QueryClientProvider,
} from '@tanstack/react-query'

import { NotificationPage } from "./components/pages/NotificationPage";

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <NotificationPage />
    </QueryClientProvider>
  )
}

export default App;
