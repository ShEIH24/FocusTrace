import { Navigate, Route, Routes } from "react-router-dom";
import { Toaster } from "sonner";
import Layout from "./components/Layout";
import Dashboard from "./pages/Dashboard";
import Timeline from "./pages/Timeline";
import Analytics from "./pages/Analytics";
import Settings from "./pages/Settings";
import DailyReport from "./pages/DailyReport";
import Focus from "./pages/Focus";

export default function App() {
  return (
    <>
      <Toaster
        theme="dark"
        position="bottom-right"
        toastOptions={{
          classNames: {
            toast: "bg-zinc-900 border border-zinc-700 text-zinc-100",
            description: "text-zinc-400",
          },
        }}
      />
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard"    element={<Dashboard />} />
          <Route path="timeline"     element={<Timeline />} />
          <Route path="analytics"    element={<Analytics />} />
          <Route path="daily-report" element={<DailyReport />} />
          <Route path="focus"        element={<Focus />} />
          <Route path="settings"     element={<Settings />} />
        </Route>
      </Routes>
    </>
  );
}
