export NODE_ENV=${NODE_ENV:=production}
export STORYBOOK_DIST=${STORYBOOK_DIST:=./dist/storybook}

if [ "$1" = "--app=vite" ]; then
  pnpm ts:check && pnpm vite build --mode $NODE_ENV
fi;
if [ "$1" = "--app=crx" ]; then
  pnpm ts:check && pnpm vite build --config vite.crx.config.ts --mode $NODE_ENV
fi;
if [ "$1" = "--app=storybook" ]; then
  pnpm ts:check && pnpm build-storybook -o $STORYBOOK_DIST
fi;
