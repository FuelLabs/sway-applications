import { Grid } from "@mui/material";

import { Cell } from ".";

export const Board = () => {
  return (
    <Grid container spacing={2} sx={{ width: "75%" }}>
      <Cell />
      <Cell />
      <Cell />
      <Cell />
      <Cell />
      <Cell />
      <Cell />
      <Cell />
      <Cell />
    </Grid>
  );
};
