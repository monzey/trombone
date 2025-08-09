import { Grid } from "@chakra-ui/react";
import type { FC } from "react";
import { Outlet } from "react-router";

export const Layout: FC = () => {
  return (
    <Grid>
      <Outlet />
    </Grid>
  )
}
