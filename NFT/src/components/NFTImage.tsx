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
      className={
        className ??
        "w-80 h-80 sm:w-64 sm:h-64 md:w-72 md:h-72 lg:w-60 lg:h-60 xl:w-80 xl:h-80"
      }
    />
  );
};
