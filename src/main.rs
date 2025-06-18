use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};


fn run (cmd: &str) { // run a given command
    println!("Running {}", cmd);
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed: {}", cmd);
        std::process::exit(1);
    }
}

fn install_brew(){
    run("which brew || sh -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"");
    
}
fn install_dependencies(os: &str){
    match os {
        "linux" => {
            run("sudo apt update");
            run("sudo apt install -y git zsh tmux alacritty rbenv");
            run("brew install neovim");
        }
        "macos" => {
            run("brew install git zsh neovim tmux rbenv");
        }
        _ => {
            eprintln!("Unsupported OS!!!");
            std::process::exit(1);
        }
    }
    run("chsh -s $(which zsh)")
}
fn install_ohmyzsh(){
    if env::var_os("ZSH").is_none() {
        run("sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"");
    } else {
        println!("OhMyZSH already installed!!")
    }
}
fn install_pyenv() {
    // Install pyenv if not already present
    let home = dirs::home_dir().expect("Could not find home directory!!!");
    let pyenv_dir = home.join(".pyenv");
    if !pyenv_dir.exists() {
        run("curl https://pyenv.run | bash");
    } else {
        println!("pyenv already installed :D");
    }
}

fn install_nerdfont() {
// install firacode nerdfont
    run("brew install --cask font-fira-code-nerd-font");
}

fn install_starship() {
    run("brew install starship"); // with starship i also want vi mode
    run ("git clone https://github.com/jeffreytse/zsh-vi-mode \
  $ZSH_CUSTOM/plugins/zsh-vi-mode")
}


fn load_dotfiles(repo_url: &str) {
    let home = dirs::home_dir().expect("No home directory found :(");
    

    let cfg_dir = home.join(".cfg");

    if cfg_dir.exists(){
        println!("Config already exists, skipping")
    } else {

    
        fs::write(home.join(".gitignore"), ".cfg\n").expect("Failed to write to .gitignore :(((");
        
        run(&format!("git clone --bare {} {}", repo_url, cfg_dir.display()));

        let checkout = Command::new("sh") // run checkout (alias is not defined yet)
           .arg("-c")
           .arg("git --git-dir=$HOME/.cfg/ --work-tree=$HOME checkout")
           .stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .output()
           .expect("Failed to run checkout");
        
        if !checkout.status.success() {
            println!("Conflict detected. Backing up clashing files.");
            run("rm $HOME/.zshrc");
        }
        // Hide untracked files
        run("git --git-dir=$HOME/.cfg/ --work-tree=$HOME config --local status.showUntrackedFiles no");
    }
}
fn generate_ssh_key() {
    let home = dirs::home_dir().expect("Could not find home directory");
    let ssh_dir = home.join(".ssh");
    if !ssh_dir.exists() {
        run("mkdir -p $HOME/.ssh");
    }
    let key_path = ssh_dir.join("id_ed25519_github");
    if key_path.exists() {
        println!("SSH key already exists at {}", key_path.display());
    } else {
        // Get user's email from git config
        let email_output = Command::new("git")
            .args(&["config", "--get", "user.email"])
            .output()
            .expect("Failed to get git user.email");
        let email = String::from_utf8_lossy(&email_output.stdout).trim().to_string();
        let cmd = format!("ssh-keygen -t ed25519 -C \"{}\" -f {} -N \"\"", email, key_path.display());
        run(&cmd);
    }
    // Print the public key
    let pub_key = ssh_dir.join("id_ed25519_github.pub");
    println!("GitHub SSH public key:");
    run(&format!("cat {}", pub_key.display()));
}

fn install_node() {
    run("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash");
    // Load nvm and install LTS version
    run("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm install --lts && nvm use --lts");
}
fn main() {
    let os = match env::consts::OS {
        "linux" => "linux",
        "macos" => "macos",
        _ => {
            eprintln!("Unsupported OS!!");
            std::process::exit(1);
        }
    };
    
    println!("Detected OS {}", os);

    install_brew();
    install_dependencies(os);
    install_ohmyzsh();
    install_pyenv();
    install_nerdfont();
    install_starship();
    load_dotfiles("https://github.com/Skyward176/.dotfiles");
    generate_ssh_key();
    install_node();
}
