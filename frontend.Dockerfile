FROM node:20-slim AS build

WORKDIR /app
COPY ./frontend .
RUN npm i

ARG BASE_URL="http://localhost/rateio"

ENV VITE_API_BASE_URL=$BASE_URL
ENV NODE_OPTIONS="--max-old-space-size=4096"
ENV CI=true
ENV NODE_ENV=production

RUN npm run build --debug


FROM nginx:1.29.4-alpine AS runner

COPY --from=build /app/dist /usr/share/nginx/html
COPY ./nginx/config.conf /etc/nginx/conf.d/default.conf

EXPOSE 80



