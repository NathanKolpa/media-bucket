FROM node:lts-alpine

RUN mkdir /var/www/html -p

WORKDIR /build
COPY . .

RUN npm install

RUN npm run build-prod

ENTRYPOINT ["cp","-r","-v","./dist/webclient/.","/var/www/html/"]
