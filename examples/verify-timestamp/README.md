# Post-quantum time-stamp: try it yourself

Sineclave Trust Node runs a public RFC 3161 time-stamp authority at

    https://trustnode.sineclave.com/tsa

A time-stamp proves that a document existed, unchanged, at a given moment. Only a SHA-384 hash of the
file is sent; the file never leaves your machine. The token is a standard RFC 3161 reply, so any
OpenSSL can check it, today and years from now. It also carries a post-quantum (ML-DSA) signature, so
the proof does not depend on classical cryptography alone.

## Run it

Linux or macOS, with `openssl` and `curl`:

```sh
./timestamp.sh contract.pdf
```

Expected output ends with `Verification: OK`, and `contract.pdf.tsr` is the token. Change one byte of
the file and the same check fails.

The token is verified against [`sineclave-trust-root.pem`](sineclave-trust-root.pem), the root
certificate kept in this repository, not against anything the server sends.
Its SHA-256 fingerprint:

    E8:7A:59:19:DF:FD:C0:24:B7:77:D4:5F:D0:62:F1:A9:79:5C:FA:02:CC:1C:18:B5:96:08:B4:17:67:06:FE:52

Check it with `openssl x509 -in sineclave-trust-root.pem -noout -fingerprint -sha256`.

<details><summary>The same steps by hand</summary>

```sh
openssl ts -query -data contract.pdf -sha384 -cert -out req.tsq
curl -s -H 'Content-Type: application/timestamp-query' --data-binary @req.tsq \
     https://trustnode.sineclave.com/tsa -o contract.pdf.tsr
openssl ts -reply -in contract.pdf.tsr -text
openssl ts -verify -data contract.pdf -in contract.pdf.tsr -CAfile sineclave-trust-root.pem
```
</details>

## Limits of the public demonstration

- 5 time-stamps per minute per IP address. Beyond that the service answers HTTP 429.
- It is a demonstration instance, not a qualified trust service under eIDAS.
- No service-level commitment. Deployments for customers run on their own infrastructure, without
  these limits.

Questions: info@marant.hr
