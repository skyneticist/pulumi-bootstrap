# Pulumi-Bootstrap Application (pulumime)  

## Overview  

**Pulumi-Bootstrap (pulumime)** is a Rust-based CLI tool distributed via `npm` to simplify the process of bootstrapping infrastructure for new applications at Vizient. By standardizing and automating setup tasks, it aims to reduce errors, save time, and provide a consistent starting point for infrastructure-as-code (IaC) projects.

This tool is specifically designed to address common pain points encountered by full-stack engineers, including:  
- Setting up secure and effective network configurations.  
- Ensuring proper communication and access between resources.  
- Adhering to best practices for hosting and access management.  
- Supporting member-facing and internal applications.  

Pulumi-Bootstrap is intended to ease these challenges by providing a standardized, flexible, and opinionated approach to infrastructure setup.  

---

## Repository Structure  (incomplete)

```bash
project-root/
├── src/                  # Rust source code for the application.
├── rust-binaries/        # Location where Rust binaries are built.
├── npm/
│   ├── bin/              # Contains the final binary (e.g., pulumi-bootstrap).
│   ├── package.json      # NPM package descriptor for distribution.
│   ├── scripts/          # Scripts for lifecycle events (preinstall, postinstall, etc.).
│   ├── move-binary.js    # Script to move the binary to the npm bin directory.
│   └── other-npm-configs/ # Additional NPM-related configuration files.
└── Cargo.toml            # Rust project descriptor file.
```

## Project Name

**Pulumi Bootstrapping**

## Repos

- pulumi-bootsrapper
- snippets (sym-linked to snippets resource in other repo)

```bash
pulumi-bootstrap/
│
├── src/
│   ├── commands/
│   │   ├── config.rs
│   │   ├── project.rs
│   │   └── snippet.rs
│   │
│   ├── tui/
│   │   ├── app.rs
│   │   ├── constants.rs
│   │   ├── enums.rs
│   │   ├── tui_main.rs
│   │   ├── ui.rs
│   │   └── validation.rs
│   │
│   ├── helpers/
│   │   ├── config_helper.rs
│   │   ├── cost_calculator.rs
│   │   ├── entrypoint_helper.rs
│   │   └── pipeline_helper.rs
│   │
│   ├── snippets/                         # This is a separate repo - git submodule 
│   │   ├── azure-pipelines.yaml
│   │   ├── cache.ts
│   │   ├── container-registry.ts
│   │   ├── database.ts
│   │   ├── keyvault.ts
│   │   ├── pulumi.webstackvzn.ts
│   │   ├── service-bus.ts
│   │   └── storage.ts
│   │
│   ├── cli.rs
│   └── main.rs
│
├── npm/
│   ├── move-binary.js
│   ├── package.json
│   ├── package-lock.json
│   └── .npmrc
│
├── azure-pipelines.yaml
├── npm_local.sh
├── cargo.toml
├── cargo.lock
├── .env
├── .gitignore
└── README.md
```

The NPM package for this tool is available at:
👉 **[Insert Artifact URL/Link Here]**

---

## Why Pulumi-Bootstrap?  

Setting up infrastructure manually or copying configurations between projects often leads to inefficiencies and potential misconfigurations. At Vizient, many full-stack engineers face challenges like:  
- Navigating complex networking requirements.  
- Managing security and access control configurations.  
- Handling member-facing and internal services consistently.  

This tool aims to:  
1. **Reduce Complexity** – Automate common setup tasks.  
2. **Save Time** – Provide ready-to-use, standardized configurations.  
3. **Promote Best Practices** – Encourage consistent patterns for IaC projects.  

### Challenges Being Addressed  

1. **Network Configuration**  
   Creating "just-enough" communication and openness between resources is critical. Pulumi-Bootstrap provides a guided approach to ensure secure, functional, and well-documented networking setups.  

2. **Lack of Standards**  
   Current workflows often involve copying configurations from other projects, which can propagate errors or inconsistencies. This tool introduces standardization without being overly prescriptive, giving teams the flexibility to extend or customize as needed.  

---

## Paradigms and Approaches  

### Declarative Style  
**Best suited for:**  
- Smaller projects.  
- Teams that prioritize simplicity and clarity in resource definitions.  

Declarative IaC provides a straightforward, "what-you-see-is-what-you-get" approach, making it easy to review and understand resource configurations at a glance.

### Object-Oriented Style  
**Best suited for:**  
- Large or growing projects.  
- Teams that require extensive reusability and modularity.  

Object-oriented Pulumi projects, while more complex, allow for scalable and maintainable infrastructure code, especially when dealing with evolving requirements.  

---

## Installation  

To install the CLI via `npm`:  

```bash
npm install -g pulumime
```

After installation, the `pulumi-bootstrap` binary will be available globally.  

---

## Usage  

Run the tool with:  
```bash
pulumime [options]
```  

For detailed help and examples:  
```bash
pulumime --help
```

---

## Contributing  (NEEDS UPDATE!)

1. Clone the repository:  
   ```bash
   git clone https://github.com/your-repo/pulumime.git
   ```  

2. Build the Rust binary:  
   ```bash
   cargo build --release
   ```  

3. Move the binary to the `npm/bin` directory (handled automatically in CI):  
   ```bash
   node npm/scripts/move-binary.js
   ```  

4. Run the CLI locally:  
   ```bash
   ./npm/bin/pulumi-bootstrap
   ```  

---

## Future Plans  

Pulumi-Bootstrap is an evolving project. Future iterations may include:  
- Enhanced support for additional infrastructure providers.  
- Templates for common application types.  
- Integration with existing Vizient workflows for seamless adoption.  

---

### Notes for Improvement  

- Ensure `move-binary.js` handles all edge cases for copying binaries across platforms.  
- Validate the effectiveness of both declarative and object-oriented paradigms in practical use cases.  

---
