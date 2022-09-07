# Let's check SPM dependency licences in terminal

---

So I'm working on reviewing and documenting all our dependencies and I've realized that there is no easy to comprehend tool for this task (unless you are a swift developer). That's why I decided to write my own tool in Rust and integrate it with our CI process. 



## Usage

---

```bash
spm-license-check

USAGE:
    spm-license-check [OPTIONS] --licencefile <LICENCEFILE> --workspace <WORKSPACE> --token <TOKEN>

OPTIONS:
    -h, --help                         Print help information
    -l, --licencefile <LICENCEFILE>    Path to the licenses TOML file
        --lowercase                    Enforce lowercase in license names
    -t, --token <TOKEN>                swiftpack.co API token (ask @ptrpavlik on twitter for one)
    -w, --workspace <WORKSPACE>        Path to the xcworkspace file
```


## Building a binary

---

- Clone the repo
  
  ```bash
  git clone *repo link will be here* && cd *repo name*
  ```

- Build the project
  
  ```bash
  cargo build --release
  ```

- The binary can be found in **./target/release/**
 

## Getting swiftpack.co API token

---

In order to obtain the swiftpack.co API token you'll have to contact @ptrpavlik on Twitter or email him [petrpavlik@me.com](mailto:petrpavlik@me.com).

