FROM node:22-alpine
WORKDIR /workspace
COPY package.json .
RUN npm install
COPY . .
CMD ["npm", "run", "build"]
