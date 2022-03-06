# cert_installer

`cert_installer` is a utility that adds a CA certificate to Android's
System Trust Store by overwriting the `/system/etc/security/cacerts` directory
with a tmpfs mount. Changes made to the System Trust Store is _not_ persistant
across reboots.

The CA certificate is valid for 360 days and needs to be updated before it
expires. Generating a new CA certificate can be done with the following
command:

```
openssl req -x509 -new -nodes -keyform der -key ca_key.der -sha256 -days 360 -outform der -out ca_cert.der -config ca.cnf
```

The current certificate has the following validity period:

```
Not Before: Mar  5 12:12:15 2022 GMT
Not After : Feb 28 12:12:15 2023 GMT
```
