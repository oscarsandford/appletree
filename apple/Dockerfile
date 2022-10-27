FROM node:lts-alpine
ENV NODE_ENV=production
ENV EDEN=http://eden:8080
WORKDIR /usr/src/app
COPY ["package.json", "package-lock.json*", "npm-shrinkwrap.json*", "./"]
RUN npm install -g typescript
RUN npm install --production --silent && mv node_modules ../
COPY . .
# EXPOSE 8080
RUN chown -R node /usr/src/app
USER node
CMD ["npm", "start"]