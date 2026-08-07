# TAK authentication flows

This document describes the authentication and transport flows implemented by the ArmaTAK Rust extension. It is development documentation, not end-user documentation.

## Security model

ArmaTAK currently exposes three TCP connection modes:

| Mode | ArmaTAK entry point | Authentication | Transport protection | Intended use |
| --- | --- | --- | --- | --- |
| Plain TCP | `tcp_socket.start` | None | None | Local development or explicitly trusted isolated networks only |
| Manual mTLS | `tcp_socket.start_mtls` | Client X.509 certificate | TLS with server certificate validation | Production-compatible secure connection |
| Credential enrollment followed by mTLS | `tcp_socket.start_enroll_mtls` | HTTP Basic credentials during enrollment, then client X.509 certificate | Enrollment HTTPS currently skips certificate validation; the resulting TAK stream uses validated mTLS | Transitional/bootstrap flow; do not treat the enrollment phase as secure until server verification is enabled |

The official TAK Server example enables a TLS input on port `8089`. Its plaintext TCP/streaming examples are anonymous and are shown on ports `8087` and `8088`. ArmaTAK must not infer security from a port number; the selected connection mode controls the client-side security properties.

## Insecure flow: plain TCP

The plain flow is deliberately simple:

```text
Arma/SQF
   |
   | tcp_socket.start("host:port")
   v
ConnectionConfig::Plain
   |
   v
TCP connect
   |
   | unencrypted CoT bytes
   v
TAK Server plaintext/anonymous input
```

`ConnectionConfig::Plain` resolves the supplied address and opens a normal `TcpStream`. No TLS handshake occurs, there is no server identity verification, and no client identity is established by ArmaTAK.

Consequences:

- CoT payloads can be observed or modified by an on-path attacker.
- ArmaTAK cannot prove that it connected to the intended TAK Server.
- The server cannot authenticate ArmaTAK by certificate.
- Plain TCP must never be the production default.

The official TAK Server `CoreConfig.example.xml` keeps anonymous TCP inputs commented out and enables the TLS input instead. Use plain TCP only when a developer intentionally configures a corresponding non-TLS server input.

## Secure flow: manual mTLS

The manual mTLS flow uses pre-provisioned certificate material:

```text
Arma/SQF
   |
   | tcp_socket.start_mtls(
   |   address,
   |   server_name,
   |   ca_cert_path,
   |   client_cert_path,
   |   client_key_path
   | )
   v
ConnectionConfig::Mtls
   |
   +--> load trusted CA certificate(s)
   +--> load client certificate chain
   +--> load client private key
   |
   v
rustls client configuration
   |
   +--> validate server certificate chain
   +--> validate TLS server name
   +--> present client certificate
   v
Authenticated mTLS TAK stream
```

The CA file is loaded into a dedicated `rustls::RootCertStore`; the client certificate and private key are used for client authentication. When `server_name` is empty, ArmaTAK derives a name from the target address. A non-empty `server_name` should be preferred when DNS and certificate names are known.

This is the preferred production flow when certificate provisioning is handled outside ArmaTAK.

## Enrollment flow: credentials to client certificate

TAK enrollment converts a username/password bootstrap credential into a short- or medium-lived client certificate, then reconnects using mTLS.

### Protocol contract

ArmaTAK follows the contract visible in the official TAK Server and ATAK implementations:

1. `GET https://HOST:ENROLL_PORT/Marti/api/tls/config`
2. Authenticate the request with HTTP Basic credentials.
3. Read `serverPort` and `enrollPath` from the TAK certificate configuration. If the fields are absent, ArmaTAK currently falls back to `8089` and `/Marti/api/tls/signClient/v2`.
4. Generate a local RSA key pair and PKCS#10 CSR. The private key never needs to be sent to the server.
5. `POST https://HOST:ENROLL_PORT{enrollPath}?clientUid=CLIENT_UID`
6. Authenticate the signing request with HTTP Basic credentials, send the CSR as `application/pkcs10`, and request JSON.
7. Parse `signedCert` and `ca0` from the enrollment response.
8. Build an in-memory client identity from the returned certificate plus the locally generated private key.
9. Connect to `HOST:serverPort` with mTLS and validate the TAK streaming server against the returned CA.

The official TAK Server exposes `/tls/config`, `/tls/signClient`, and `/tls/signClient/v2`. The v2 endpoint can return JSON containing `signedCert` plus `ca0`, `ca1`, and additional CA-chain entries. The current ArmaTAK implementation consumes `signedCert` and `ca0`.

### Current ArmaTAK enrollment security caveat

`enrollment_http_client()` currently uses `danger_accept_invalid_certs(true)`. Therefore the HTTPS bootstrap does **not** authenticate the enrollment server certificate.

That means the current enrollment phase is vulnerable to an active man-in-the-middle attacker who can impersonate the HTTPS enrollment endpoint and receive the submitted Basic credentials. The private key remains local, but the username/password bootstrap secret is still exposed to the impersonating endpoint.

The connection established *after* enrollment is mTLS and uses the returned CA, but that does not retroactively secure the bootstrap exchange. Consequently:

- `start_enroll_mtls` is a transitional flow, not the preferred production-secure flow in its current form.
- `start_mtls` with a trusted CA and pre-provisioned client certificate is the secure production path today.
- A future secure enrollment implementation must validate the HTTPS server with a trusted CA, certificate pin, or equivalent authenticated trust bootstrap.
- An explicit insecure/development enrollment switch is preferable to silently disabling verification.

The official ATAK enrollment manager exposes a `verifyHost` choice and records whether host verification is on or off. ArmaTAK should converge on the same explicit distinction.

## Target secure enrollment design

The intended production enrollment flow is:

```text
username/password
      |
      v
HTTPS enrollment endpoint
(server certificate MUST validate)
      |
      +--> GET /Marti/api/tls/config
      +--> generate local private key + CSR
      +--> POST configured signClient endpoint
      v
signed client certificate + trusted CA
      |
      v
mTLS TAK streaming connection
      |
      +--> server authenticated by CA/name
      +--> client authenticated by certificate
      v
CoT traffic
```

The development-only insecure variant should be explicit:

```text
username/password
      |
      v
HTTPS enrollment endpoint
(certificate verification explicitly disabled)
      |
      v
WARNING / development-only path
```

Do not silently fall back from verified enrollment to unverified enrollment after a certificate validation failure.

## Secret-handling requirements

- Never include passwords, private keys, certificate payloads, or Authorization headers in logs.
- `ConnectionConfig::describe()` may include the username and client UID for diagnostics, but it must never include the password.
- Keep generated private keys in memory unless a future feature explicitly requires persistence.
- Do not commit development certificates or credentials to the repository.
- Treat enrollment credentials as bootstrap secrets and rotate/revoke them according to the TAK Server deployment policy.

## Test contract

Unit tests should lock down the protocol-level behavior that does not require a live TAK Server:

- official endpoint paths and default mTLS port;
- trimming and construction of enrollment URLs;
- parsing TAK enrollment configuration with explicit values and fallback defaults;
- PEM normalization;
- absence of passwords from diagnostic descriptions;
- TLS server-name derivation and PEM parsing where deterministic;
- CoT and MAVLink pure serialization logic.

Live certificate signing, TLS handshake behavior, socket reconnect behavior, mDNS, video processes, and Arma callback behavior belong in integration/E2E tests because they depend on operating-system or external-server state.

## Upstream references

The tests and protocol assumptions in this repository are based on these upstream implementations:

- TAK Server, `CertManagerApi.java`: <https://github.com/TAK-Product-Center/Server/blob/5187abd46d827d37cfc5708805eced197a837e49/src/takserver-core/takserver-war/src/main/java/com/bbn/tak/tls/CertManagerApi.java>
- TAK Server, `CoreConfig.example.xml`: <https://github.com/TAK-Product-Center/Server/blob/5187abd46d827d37cfc5708805eced197a837e49/src/takserver-core/example/CoreConfig.example.xml>
- ATAK/CommonCommo, `enrollmentmanager.cpp`: <https://github.com/TAK-Product-Center/atak-civ/blob/9f6893dd657feacc35ec5de03dad721c2e44170e/commoncommo/core/impl/enrollmentmanager.cpp>

Pinning the source revisions in this document keeps the development contract reproducible. When the upstream protocol changes, update both the implementation/tests and these references together.
