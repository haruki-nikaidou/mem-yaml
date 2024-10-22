# Mem YAML

Anki's alternative for who hates GUI and mouse clicks.

*This project is in early stage. If there are more than 5 users, I'll finish it.*

## Installation

### Install with cargo:

```sh
cargo install mem-yaml
```

### Install with nix flakes:

in `flake.nix`

```nix
{
    inputs.mem-yaml.url = "github:haruki-nikaidou/mem-yaml";
}
```

in `configuration.nix`

```nix
{ pkgs, inputs, ... }:
{
    environment.systemPackages = with pkgs; [
        inputs.mem-yaml.packages.${pkgs.system}.mem-yaml
    ];
}
```

or, if you use home manager
    
```nix
{ pkgs, inputs, ... }:
{
    home.packages = with pkgs; [
        inputs.mem-yaml.packages.${pkgs.system}.mem-yaml
    ];
}
```