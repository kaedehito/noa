# noa Package Manager

### Overview
noa is a minimal and powerful package manager designed for Noa Linux, inspired by Gentoo's Portage. noa aims to be the core system manager, handling not only package management but also init system tasks and service management.

### Features
- Source-based package management
- Highly customizable build options
- Integrated init system and service management
- Dependency tracking and resolution
- Simple and lightweight design

### Installation
To install noa, follow these steps:

```sh
# Clone the repository
git clone https://github.com/kaedehito/noa.git
cd noa

# Run the installation script
./install.sh
```

### Usage
Basic usage examples:

```sh
# Install a package
noa install package-name

# Remove a package
noa remove package-name

# Update the system
noa update
```

For more commands, run:
```sh
noa --help
```

### Contributing
Contributions are welcome! Please submit issues and pull requests on the GitHub repository.

### License
noa is licensed under the MIT License.

