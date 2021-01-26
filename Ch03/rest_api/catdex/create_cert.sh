openssl req -x509 -newkey rsa:4096 \
  -keyout key.pem \
  -out cert.pem \
  -days 365 \
  -sha256 \
  -subj "/CN=localhost"

openssl rsa -in key.pem -out key-no-password.pem
