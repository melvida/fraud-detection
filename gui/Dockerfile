FROM node:18-alpine as builder

WORKDIR /app

# Install pnpm globally
RUN npm install -g pnpm

COPY package.json pnpm-lock.yaml ./

# Use pnpm instead of npm
RUN pnpm install --frozen-lockfile

COPY public public
COPY src src

RUN pnpm run build

# Runtime stage
FROM node:18-alpine

WORKDIR /app

RUN npm install -g serve pnpm

COPY --from=builder /app/build ./build

EXPOSE 3000

CMD ["serve", "-s", "build", "-l", "3000"]