FROM node:19

WORKDIR /application

COPY ./ui/app/package.json .

RUN npm install pnpm --global

RUN pnpm install

COPY ./ui/app .

EXPOSE 8080

CMD [ "pnpm", "run", "dev", "--host" ]
