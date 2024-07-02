import { Card, CardActionArea, CardContent } from "@mui/material";
import { Text } from "./Text";
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
          <Text className="text-4xl mb-4">{title}</Text>
          <Text className="text-base">
            {children}
          </Text>
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
