./target/release/lucid init
cat > /app/.config/lucid/lucid.yml <<EOL
---
default:
  bind_address: 0.0.0.0
  port: ${PORT}
  port_ssl: ${PORT}
  use_ssl: false
  ssl_certificate: "tls/cert.pem"
  ssl_certificate_key: "tls/key.rsa"
authentication:
  enabled: false
  root_token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJMdWNpZCBSb290IFRva2VuIiwiaXNzIjoiaHR0cDovLzEyNy4wLjAuMTo3MDIxLyIsImlhdCI6MTU3NDQyMDE0NSwiZXhwIjoxNjY4NzY4OTQ1fQ.iGGRmZXjwsO4PSQKNI1qGtsA1Sj94SBXB4WU_XN5EuQ
  secret_key: 03dfe6e600e2620388311955ab3bee32cd6cbb8aca75dace6cfc2e2a43db5dfd
persistence:
  enabled: false
  location: ""
encryption:
  enabled: false
  private_key: ""
webui:
  enabled: false
store:
  max_limit: 7340032
http:
  request_size_limit: 8388608
logging:
  level: Info
EOL
./target/release/lucid server