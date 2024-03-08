import { Grid, Paper, Typography } from "@mui/material";
import { useState } from "react";

export const Cell = () => {
   const [text, setText] = useState<"X" | "O" | null>();

   return (
   <Grid item xs={4}>
        <Paper variant="outlined" sx={{ height: "150px", alignItems: "center", justifyContent: "center", display: "flex" }}>
         <Typography sx={{ fontSize: "150px" }}>{text}</Typography>
        </Paper>
    </Grid>
   );
}
