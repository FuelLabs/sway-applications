import { Card, CardActionArea, CardContent, Typography } from "@mui/material";
import { useRouter } from "next/router";

type HomeCardProps = {
  href: string;
  title: string;
  children: React.ReactNode
};

export const HomeCard = ({ href, title, children }: HomeCardProps) => {
  const router = useRouter();

  return (
    <Card
      variant="outlined"
      sx={{
        padding: "18px",
        backgroundColor: "black",
        borderColor: "#1e1e1e",
        borderWidth: "3px",
        height: "142px",
        width: "280px"
      }}
      className="hover:bg-slate-950"
    >
      <CardActionArea
        onClick={() => {
          router.push(href);
        }}
        sx={{ height: "stretch" }}
      >
        <CardContent>
          <Typography className="text-white font-sans text-4xl mb-4">{title}</Typography>
          <Typography className="text-white font-sans text-base">
            {children}
          </Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
