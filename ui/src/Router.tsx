import type { FC } from "react";
import { Route, Routes } from "react-router";
import { Layout } from "./components/Layout";
import { Overview } from "./pages/Overview";

export const Router: FC = () => {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<Overview />} />
      </Route>
    </Routes>
  )
}
