# Build Stage
FROM node:26-bullseye-slim AS build

WORKDIR /app

COPY package*.json ./
COPY pnpm-lock.yaml* ./

RUN npm install -g pnpm@10.18.3

RUN pnpm i

COPY . .
RUN pnpm run build


# Runtime Stage
FROM nginx:alpine

COPY --from=build /app/dist /usr/share/nginx/html

# optional aber sinnvoll
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]