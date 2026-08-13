# Palantir

**Palantir** is a lightweight Kubernetes dashboard application built using [Rust](https://www.rust-lang.org/), [actix-web](https://actix.rs/), and [quench-web](https://github.com/lorehaven/forge). It provides an intuitive, real-time web interface to observe and manage your Kubernetes clusters with speed and elegance.

---

## 🚀 Features

- 📊 Visualize Kubernetes resources
- 🔄 Live updates via [htmx](https://htmx.org/) polling fragments - no client-side JavaScript framework
- 🛡️ Secure by design – runs locally or in-cluster
- 🔌 Extendable and modular architecture

---

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) – backend logic and performance
- [actix-web](https://actix.rs/) – HTTP server
- [quench-web](https://github.com/lorehaven/forge) – server-rendered HTML builder, shared across every `forge` service
- [htmx](https://htmx.org/) – the small amount of client-side interactivity (polling, form submission) each page needs

---

## 📦 Installation

> ⚠️ **Requirements**: Rust toolchain, `sass` (`npm install -g sass`)

Clone the repo:

```bash
git clone https://github.com/lorehaven/palantir.git
cd palantir
```

Build and run the server:

```bash
mkdir -p dist/assets/css
sass --no-source-map styles/main.scss dist/assets/css/palantir.css
cargo run -p server
```

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

`run.sh` compiles `styles/*.scss` and starts the server with local-dev defaults
(in-memory session/cache store, auth disabled):

```bash
./run.sh
```

There's no client-side build/hot-reload step - every page is rendered server-side,
so a plain `cargo watch -x 'run -p server'` (or just re-running `run.sh`) picks up
Rust changes; edits to `styles/*.scss` need the `sass` command re-run to take effect.

---

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you’d like to change.

- Make sure to format code with \`cargo fmt\`
- Run \`cargo clippy --all-targets --all-features -- -D warnings\` before submitting

---

## 📄 License

[License](LICENSE)

---
