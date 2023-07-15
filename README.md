Generate SSL certificate 

```shell
openssl genrsa -traditional -out key.pem
openssl req -new -x509 -key key.pem -out cert.pem -config openssl.cnf -days 9999

```
