# Pulumi-Bootstrap Application (pulumimi)  

## Overview  

**Pulumi-Bootstrap (pulumimi)** is a Rust-based CLI tool distributed via `npm` to simplify the process of bootstrapping infrastructure for new applications at Vizient. By standardizing and automating setup tasks, it aims to reduce errors, save time, and provide a consistent starting point for infrastructure-as-code (IaC) projects.

This tool is specifically designed to address common pain points encountered by full-stack engineers, including:

- Setting up secure and effective network configurations.

- Ensuring proper communication and access between resources.  

- Adhering to best practices for hosting and access management.

- Supporting member-facing and internal applications.  

Pulumi-Bootstrap is intended to ease these challenges by providing a standardized, flexible, and opinionated approach to infrastructure setup. (? - Review this)

## Note

### Integration with Vizient Pulumi Snippets

**Vizient Pulumi Snippets** is a Vizient repository... (need to fill out).

Pulumi-Bootstrap leverages the **Vizient Pulumi Snippets** repository as a git submodule to provide reusable infrastructure components. This integration ensures that common configurations and best practices are easily accessible and maintainable.
### Available via `npm`

Pulumi-Bootstrap is a Rust-based CLI and TUI application. This project uses **NPM** for distribution, since most Vizient engineers have Node and NPM installed on their machines. It can also be downloaded directly from the Vizient repository: <ADD_REPO_HERE>. Alternatively, you can use Rust and Cargo. To do so, clone the `Pulumi-Bootstrap` repository and follow the standard procedure for running Rust programs.

###

---

## Repository Structure

## Repos

- pulumi-bootsrap
- Vizient Pulumi Snippets
    (used as git submodule in pulumi-bootstrap)

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
├── snippets/ (independent repo used as git submodule)
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

[
   !!! MUST BE UPDATED !!!
]
## Installation  

To install the CLI via `npm`:  

```bash
npm install -g pulumimi
```

After installation, the `pulumi-bootstrap` binary will be available globally.  

---

## Usage  

Run the tool with:  
```bash
pulumimi [options]
```  

For detailed help and examples:  
```bash
pulumimi --help
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
