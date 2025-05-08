FROM nginx:alpine
LABEL authors="paulschilling"

RUN apk add --no-cache bash
COPY nginx.conf /etc/nginx/conf.d/default.conf

COPY wait-for-it.sh /wait-for-it.sh
RUN chmod +x wait-for-it.sh

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x entrypoint.sh

CMD ["/entrypoint.sh"]