# For starting the dev server
FROM node:15.14.0-alpine
WORKDIR /app
COPY package.json .
RUN npm install
COPY . .
EXPOSE 8080
CMD ["npm", "run", "start"]
