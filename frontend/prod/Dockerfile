FROM nginx:alpine
LABEL authors="paulschilling"

COPY ../dev/nginx.conf /etc/nginx/conf.d/default.conf

COPY ./static /usr/share/nginx/html