# The browser version, served by nginx: the site of a release's
# gadget-generator-web.zip (the CI's docker job unpacks it next to this
# file's context). No game data: players bring their CD image or play
# without it.
#
#   docker run -p 8080:80 ghcr.io/doridian/gadgetgenerator   (or :1.2.3, a release)
#   (then http://localhost:8080/)
FROM nginx:1.27-alpine
COPY gadget-generator-web/ /usr/share/nginx/html/
EXPOSE 80
