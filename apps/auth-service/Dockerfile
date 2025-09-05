# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG NODE_VERSION=24.5.0

################################################################################
# Use node image for base image for all stages.
FROM node:${NODE_VERSION}-alpine AS base

# Set working directory for all build stages.
WORKDIR /usr/src/app

# Install pnpm.
ARG PNPM_VERSION=10.13.1
RUN --mount=type=cache,target=/root/.npm \
    npm install -g pnpm@${PNPM_VERSION}

################################################################################
# Create a stage for installing production dependecies.
FROM base AS deps

# Download dependencies as a separate step to take advantage of Docker's caching.
# Leverage a cache mount to /root/.local/share/pnpm/store to speed up subsequent builds.
# Leverage bind mounts to package.json and pnpm-lock.yaml to avoid having to copy them
# into this layer.
RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
    --mount=type=bind,source=pnpm-workspace.yaml,target=pnpm-workspace.yaml \
    --mount=type=bind,source=apps/auth-service/package.json,target=apps/auth-service/package.json \
    pnpm install --prod --frozen-lockfile

################################################################################
# Create a stage for building the application.
FROM base AS build

# Copy package.json so that package manager commands can be used.
# Copy pnpm-workspace.yaml to ensure workspace dependencies & name resolution are handled.
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml turbo.json ./
COPY apps/auth-service/package.json ./apps/auth-service/package.json

# Download additional development dependencies before building, as some projects require
# "devDependencies" to be installed to build. If you don't need this, remove this step.
RUN --mount=type=cache,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile

# Copy the rest of the source files into the image.
COPY apps/auth-service/ ./apps/auth-service/
COPY packages/ ./packages/

# Run the build script.
RUN pnpm turbo run build --filter @ponzu/auth-service

################################################################################
# Create a new stage to run the application with minimal runtime dependencies
# where the necessary files are copied from the build stage.
FROM base AS final

# Use production node environment by default.
ENV NODE_ENV=production

# Run the application as a non-root user.
USER node

# Copy package.json so that package manager commands can be used.
COPY package.json .

# Copy the production dependencies from the deps stage and also
# the built application from the build stage into the image.
COPY --from=deps /usr/src/app/node_modules ./node_modules
COPY --from=deps /usr/src/app/apps/auth-service/node_modules ./apps/auth-service/node_modules
COPY --from=build /usr/src/app/apps/auth-service/package.json ./apps/auth-service/package.json
COPY --from=build /usr/src/app/apps/auth-service/dist ./apps/auth-service/dist

# Expose the port that the application listens on.
EXPOSE 8080

# Run the application.
CMD ["node", "apps/auth-service/dist/main.js"]
