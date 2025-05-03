import os
import sys
import json
import subprocess
import platform
import re
from pathlib import Path

class CopilotAutoFixer:
    def __init__(self):
        self.dependencies = {
            "gh": self.check_gh_cli,
            "github-copilot-cli": self.check_copilot_cli,
            "jq": self.check_jq
        }
        self.repo_info = self.get_repo_info()

    def get_repo_info(self):
        """Extract repository owner and name from git remote URL."""
        try:
            remote_url = subprocess.check_output(
                ["git", "config", "--get", "remote.origin.url"],
                text=True
            ).strip()

            # Extract owner and repo from different URL formats
            # https://github.com/owner/repo.git or git@github.com:owner/repo.git
            match = re.search(r'github\.com[:/]([^/]+)/([^.]+)', remote_url)
            if match:
                return {"owner": match.group(1), "name": match.group(2)}
            else:
                print("Could not parse GitHub repository information from git remote URL.")
                return None
        except subprocess.CalledProcessError:
            print("Error getting git remote URL. Make sure you're in a git repository.")
            return None

    def run(self):
        """Main execution flow."""
        print("Copilot Auto-Fix Tool")
        print("=====================")

        # Check and install dependencies
        if not self.check_dependencies():
            return False

        # Authenticate with GitHub and Copilot CLI
        if not self.ensure_authentication():
            return False

        # Fetch issues and security alerts
        issues = self.fetch_issues()
        scans = self.fetch_security_scans()

        # Process issues and scans with Copilot
        if issues:
            self.process_issues(issues)

        if scans:
            self.process_security_scans(scans)

        print("\nCopilot auto-fix tool completed!")
        return True

    def check_dependencies(self):
        """Check all required dependencies and offer to install them."""
        print("\nChecking dependencies...")
        all_ok = True

        for dep_name, check_func in self.dependencies.items():
            if not check_func():
                all_ok = False

        return all_ok

    def check_gh_cli(self):
        """Check if GitHub CLI is installed and offer to install it."""
        if self.is_command_available("gh"):
            print("✓ GitHub CLI is installed")
            return True

        print("✗ GitHub CLI is not installed")
        if self.prompt_yes_no("Would you like to install GitHub CLI now?"):
            system = platform.system().lower()

            if system == "windows":
                if self.is_command_available("scoop"):
                    self.run_command("scoop install gh")
                elif self.is_command_available("choco"):
                    self.run_command("choco install gh -y")
                else:
                    print("Please install GitHub CLI manually from: https://cli.github.com/")
                    return False
            elif system == "darwin":  # macOS
                if self.is_command_available("brew"):
                    self.run_command("brew install gh")
                else:
                    print("Please install GitHub CLI manually from: https://cli.github.com/")
                    return False
            elif system == "linux":
                if self.is_command_available("apt-get"):
                    commands = [
                        "curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg",
                        "sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg",
                        "echo \"deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main\" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null",
                        "sudo apt update",
                        "sudo apt install gh -y"
                    ]
                    for cmd in commands:
                        if not self.run_command(cmd):
                            return False
                elif self.is_command_available("dnf"):
                    commands = [
                        "sudo dnf install 'dnf-command(config-manager)' -y",
                        "sudo dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo",
                        "sudo dnf install gh -y"
                    ]
                    for cmd in commands:
                        if not self.run_command(cmd):
                            return False
                else:
                    print("Please install GitHub CLI manually from: https://cli.github.com/")
                    return False
            else:
                print(f"Unsupported operating system: {system}")
                return False

            # Verify installation
            if self.is_command_available("gh"):
                print("✓ GitHub CLI installed successfully")
                return True
            else:
                print("✗ Failed to install GitHub CLI")
                return False
        else:
            print("GitHub CLI is required for this tool.")
            return False

    def check_copilot_cli(self):
        """Check if GitHub Copilot CLI is installed and offer to install it."""
        if self.is_command_available("github-copilot-cli"):
            print("✓ GitHub Copilot CLI is installed")
            return True

        print("✗ GitHub Copilot CLI is not installed")
        if self.prompt_yes_no("Would you like to install GitHub Copilot CLI now?"):
            if self.is_command_available("npm"):
                return self.run_command("npm install -g @githubnext/github-copilot-cli")
            else:
                print("npm is not installed. Please install Node.js and npm first.")
                return False
        else:
            print("GitHub Copilot CLI is required for this tool.")
            return False

    def check_jq(self):
        """Check if jq is installed and offer to install it."""
        if self.is_command_available("jq"):
            print("✓ jq is installed")
            return True

        print("✗ jq is not installed")
        if self.prompt_yes_no("Would you like to install jq now?"):
            system = platform.system().lower()

            if system == "windows":
                if self.is_command_available("scoop"):
                    return self.run_command("scoop install jq")
                elif self.is_command_available("choco"):
                    return self.run_command("choco install jq -y")
                else:
                    print("Please install jq manually from: https://stedolan.github.io/jq/download/")
                    return False
            elif system == "darwin":  # macOS
                if self.is_command_available("brew"):
                    return self.run_command("brew install jq")
                else:
                    print("Please install jq manually from: https://stedolan.github.io/jq/download/")
                    return False
            elif system == "linux":
                if self.is_command_available("apt-get"):
                    return self.run_command("sudo apt-get install jq -y")
                elif self.is_command_available("dnf"):
                    return self.run_command("sudo dnf install jq -y")
                else:
                    print("Please install jq manually from: https://stedolan.github.io/jq/download/")
                    return False
            else:
                print(f"Unsupported operating system: {system}")
                return False
        else:
            print("jq is required for this tool.")
            return False

    def ensure_authentication(self):
        """Ensure authenticated with GitHub CLI and Copilot CLI."""
        print("\nChecking authentication...")

        # Check GitHub CLI auth
        gh_auth = self.run_command("gh auth status", capture_output=True, check=False)
        if gh_auth.returncode != 0:
            print("Not authenticated with GitHub CLI.")
            if self.prompt_yes_no("Would you like to authenticate now?"):
                print("\nSelect authentication method:")
                print("1) Browser (recommended)")
                print("2) Token")
                print("3) SSH key")

                option = input("Select option (1-3): ")

                if option == "1":
                    print("Opening browser for authentication...")
                    self.run_command("gh auth login -w")
                elif option == "2":
                    print("Please create a Personal Access Token with appropriate permissions at:")
                    print("https://github.com/settings/tokens/new")
                    print("Recommended scopes: repo, read:org, workflow")
                    token = input("Enter your GitHub Personal Access Token: ")
                    proc = subprocess.Popen(["gh", "auth", "login", "--with-token"],
                                           stdin=subprocess.PIPE,
                                           text=True)
                    proc.communicate(input=token)
                    if proc.returncode != 0:
                        print("Authentication failed!")
                        return False
                elif option == "3":
                    print("Using SSH authentication...")
                    self.run_command("gh auth login --git-protocol ssh")
                else:
                    print("Invalid option. Using default browser authentication...")
                    self.run_command("gh auth login -w")

                # Verify authentication succeeded
                gh_auth = self.run_command("gh auth status", capture_output=True, check=False)
                if gh_auth.returncode != 0:
                    print("GitHub authentication failed!")
                    return False
                print("GitHub authentication successful!")
            else:
                print("Authentication required. Exiting.")
                return False
        else:
            print("✓ Authenticated with GitHub CLI")

        # Check Copilot CLI auth
        copilot_auth = self.run_command("github-copilot-cli auth status",
                                      capture_output=True, check=False)
        if copilot_auth.returncode != 0:
            print("Not authenticated with GitHub Copilot CLI.")
            if self.prompt_yes_no("Would you like to authenticate now?"):
                print("Initiating GitHub Copilot CLI authentication...")
                print("You will need to complete the authentication process.")
                print("You may be asked to:")
                print("1. Sign in to GitHub")
                print("2. Enter a one-time code")
                print("3. Authorize Copilot CLI")

                self.run_command("github-copilot-cli auth")

                # Verify authentication succeeded
                copilot_auth = self.run_command("github-copilot-cli auth status",
                                             capture_output=True, check=False)
                if copilot_auth.returncode != 0:
                    print("Copilot authentication failed!")
                    return False
                print("Copilot authentication successful!")
            else:
                print("Authentication required. Exiting.")
                return False
        else:
            print("✓ Authenticated with GitHub Copilot CLI")

        return True

    def fetch_issues(self):
        """Fetch open issues from the repository."""
        if not self.repo_info:
            return []

        repo = f"{self.repo_info['owner']}/{self.repo_info['name']}"
        print(f"\nFetching open issues from {repo}...")

        issues_file = "issues.json"
        self.run_command(f"gh issue list --repo {repo} --state open --json number,title > {issues_file}")

        if os.path.exists(issues_file):
            with open(issues_file, 'r') as f:
                try:
                    issues = json.load(f)
                    print(f"Found {len(issues)} open issues")
                    return issues
                except json.JSONDecodeError:
                    print("Error parsing issues, possibly none exist.")
                    return []
        return []

    def fetch_security_scans(self):
        """Fetch security scans from the repository."""
        if not self.repo_info:
            return []

        repo = f"{self.repo_info['owner']}/{self.repo_info['name']}"
        print(f"\nFetching security alerts from {repo}...")

        scans_file = "scans.json"
        try:
            repo_path = f"repos/{self.repo_info['owner']}/{self.repo_info['name']}/code-scanning/alerts"
            self.run_command(f"gh api {repo_path} --paginate --jq '.[] | {{number: .number, rule: .rule, state: .state}}' > {scans_file}")

            if os.path.exists(scans_file):
                with open(scans_file, 'r') as f:
                    try:
                        content = f.read()
                        if not content.strip():  # Check if file is empty
                            print("No security alerts found.")
                            return []

                        # Parse line-delimited JSON
                        scans = []
                        for line in content.strip().split('\n'):
                            if line.strip():
                                scans.append(json.loads(line))

                        print(f"Found {len(scans)} security alerts")
                        return scans
                    except json.JSONDecodeError:
                        print("Error parsing security alerts.")
                        return []
        except subprocess.CalledProcessError:
            print("Error fetching security alerts, possibly none exist or you lack permissions.")
            return []

        return []

    def process_issues(self, issues):
        """Process each issue with Copilot."""
        print("\nProcessing issues...")

        # Make sure we're on the main branch
        self.run_command("git fetch origin")
        self.run_command("git checkout main")
        self.run_command("git pull origin main")

        for issue in issues:
            issue_number = issue.get('number')
            issue_title = issue.get('title')

            if not issue_number or not issue_title:
                continue

            print(f"\nWorking on issue #{issue_number}: {issue_title}")

            # Create a branch for this issue
            branch = f"autofix/issue-{issue_number}"

            # Check if branch already exists
            branch_exists = self.run_command(f"git show-ref --verify --quiet refs/heads/{branch}",
                                          check=False).returncode == 0
            if branch_exists:
                print(f"Branch {branch} already exists, deleting it")
                self.run_command(f"git branch -D {branch}", check=False)

            self.run_command(f"git checkout -b {branch}")

            # Try to fix using Copilot CLI
            print("Asking Copilot for suggestions...")
            self.run_command(f"github-copilot-cli suggest --fix --issue \"{issue_title}\"")

            # Check if there are changes
            git_status = self.run_command("git status --porcelain", capture_output=True).stdout
            if git_status.strip():
                print("Changes detected! Committing...")
                self.run_command("git add .")
                self.run_command(f"git commit -m \"fix: Issue #{issue_number} [auto-copilot]\"")

                # Ask user if they want to push and create PR
                if self.prompt_yes_no("Push changes and create PR?"):
                    self.run_command(f"git push --set-upstream origin {branch} --force")
                    repo = f"{self.repo_info['owner']}/{self.repo_info['name']}"
                    self.run_command(
                        f"gh pr create --title \"Auto Copilot Fix: Issue #{issue_number}\" "
                        f"--body \"This PR fixes issue #{issue_number} using Copilot CLI.\" "
                        f"--repo {repo}"
                    )
                    print("PR created successfully!")
            else:
                print(f"No changes suggested by Copilot for issue #{issue_number}")

            # Return to main branch
            self.run_command("git checkout main")

    def process_security_scans(self, scans):
        """Process each security scan with Copilot."""
        print("\nProcessing security scans...")

        for scan in scans:
            scan_number = scan.get('number')
            scan_rule = scan.get('rule', {}).get('id')
            scan_state = scan.get('state')

            if not scan_number or not scan_rule:
                continue

            if scan_state != "open":
                print(f"Scan alert #{scan_number} is not open, skipping.")
                continue

            print(f"\nWorking on security alert #{scan_number}: {scan_rule}")

            # Create a branch for this scan
            branch = f"autofix/scan-{scan_number}"

            # Check if branch already exists
            branch_exists = self.run_command(f"git show-ref --verify --quiet refs/heads/{branch}",
                                          check=False).returncode == 0
            if branch_exists:
                print(f"Branch {branch} already exists, deleting it")
                self.run_command(f"git branch -D {branch}", check=False)

            self.run_command(f"git checkout -b {branch}")

            # Try to fix using Copilot CLI
            print("Asking Copilot for suggestions...")
            self.run_command(f"github-copilot-cli suggest --fix --alert \"{scan_rule}\"")

            # Check if there are changes
            git_status = self.run_command("git status --porcelain", capture_output=True).stdout
            if git_status.strip():
                print("Changes detected! Committing...")
                self.run_command("git add .")
                self.run_command(f"git commit -m \"fix: Security Alert #{scan_number} [auto-copilot]\"")

                # Ask user if they want to push and create PR
                if self.prompt_yes_no("Push changes and create PR?"):
                    self.run_command(f"git push --set-upstream origin {branch} --force")
                    repo = f"{self.repo_info['owner']}/{self.repo_info['name']}"
                    self.run_command(
                        f"gh pr create --title \"Auto Copilot Fix: Security Alert #{scan_number}\" "
                        f"--body \"This PR fixes security alert #{scan_number} using Copilot CLI.\" "
                        f"--repo {repo}"
                    )
                    print("PR created successfully!")
            else:
                print(f"No changes suggested by Copilot for scan alert #{scan_number}")

            # Return to main branch
            self.run_command("git checkout main")

    # Utility methods
    def is_command_available(self, command):
        """Check if a command is available on the system."""
        try:
            subprocess.run([command, "--version"],
                           stdout=subprocess.PIPE,
                           stderr=subprocess.PIPE,
                           check=False)
            return True
        except FileNotFoundError:
            return False

    def run_command(self, command, capture_output=False, check=True):
        """Run a shell command and handle errors."""
        print(f"> {command}" if not capture_output else "")
        try:
            if capture_output:
                result = subprocess.run(command, shell=True,
                                      check=check,
                                      stdout=subprocess.PIPE,
                                      stderr=subprocess.PIPE,
                                      text=True)
                return result
            else:
                subprocess.run(command, shell=True, check=check)
                return True
        except subprocess.CalledProcessError as e:
            print(f"Command failed: {e}")
            if check:
                sys.exit(1)
            return False

    def prompt_yes_no(self, question):
        """Prompt user for yes/no response."""
        response = input(f"{question} (y/n): ").lower().strip()
        return response == "y" or response == "yes"

# Clean up temporary files
def cleanup():
    for file in ["issues.json", "scans.json"]:
        if os.path.exists(file):
            os.remove(file)

if __name__ == "__main__":
    try:
        fixer = CopilotAutoFixer()
        fixer.run()
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
    finally:
        cleanup()
