FROM node:23.3.0-alpine3.20
WORKDIR /app
COPY ./frontend .
RUN apk add --no-cache openssl
ENV NUXT_PUBLIC_API_BASE="."
RUN npm config set strict-ssl false
RUN npm install
RUN npx nuxi generate