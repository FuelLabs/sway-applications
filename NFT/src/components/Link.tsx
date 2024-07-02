import NextLink from "next/link";

export const Link = ({
  href,
  children,
  className,
  target,
}: {
  href: string;
  children: React.ReactNode;
  className?: string;
  target?: string;
}) => {
  return (
    <NextLink
      href={href}
      className={`text-white no-underline font-sans hover:underline hover:text-green-500 ${className}`}
      target={target}
    >
      {children}
    </NextLink>
  );
};
