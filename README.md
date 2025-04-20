# Palantir

**Palantir** is a lightweight Kubernetes dashboard application built using [Rust](https://www.rust-lang.org/) and [Leptos](https://leptos.dev/). It provides an intuitive, real-time web interface to observe and manage your Kubernetes clusters with speed and elegance.

---

## 🚀 Features

- 📊 Visualize Kubernetes resources
- 🔄 Live updates using efficient client-server communication
- 🌈 Clean, reactive UI powered by Leptos
- 🛡️ Secure by design – runs locally or in-cluster
- 🔌 Extendable and modular architecture

---

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) – backend logic and performance
- [Leptos](https://leptos.dev/) – fullstack reactive web UI in Rust

---

## 📦 Installation

> ⚠️ **Requirements**: Rust toolchain, Leptos CLI

Clone the repo:

```bash
git clone https://github.com/lorehaven/palantir.git
cd palantir
``````

Build and run the server:

``````bash
cargo leptos build
``````

Open your browser at [http://localhost:3000](http://localhost:3000)

---

## 🔧 Configuration

Palantir uses environment variables for configuration:

- SERVER_HOST [String] - server address. defaults to localhost \
- SERVER_DNS_NAME [String] - server display name. defaults to localhost \
- KUBERNETES_TOKEN_PATH [String] - for local builds - location of a file holding access token \
- ADDITIONAL_SERVICES [List<Map>] - services to be added to facade view not being a part of kubernetes cluster \
> [{name: String, url: String, url_display: String, available: bool}]
---

## 🌐 Deployment

Palantir can be containerized and deployed directly into your cluster.

The \`Dockerfile\` is provided: [Dockerfile](Dockerfile)

> While deploying inside a kubernetes cluster,\
> Consider running it with minimal RBAC privileges for read-only access if you just want a viewer mode.

---

## 🧪 Development

Start in dev mode with hot reload:

``````bash
LEPTOS_ENV=dev cargo leptos watch
``````

Frontend is auto-recompiled with changes using Leptos' hot reload.

---

---

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you’d like to change.

- Make sure to format code with \`cargo fmt\`
- Run \`cargo clippy --all-targets --all-features -- -D warnings\` before submitting

---

## 📄 License

[License](LICENSE)

---
