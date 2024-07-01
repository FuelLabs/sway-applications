import clsx from "clsx";

export const NFTImage = ({
  src,
  className,
}: {
  src: string;
  className?: string;
}) => {
  return (
    <img
      src={src}
      className={className ?? "w-80 h-80 lg:w-72 lg:h-72"}
    />
  );
};
