import subprocess
import sys

def get_yubikey_serial():
    try:
        # Run ykman info command and capture output
        result = subprocess.run(['ykman', 'info'],
                              capture_output=True,
                              text=True)

        if result.returncode != 0:
            raise Exception("Error running ykman: " + result.stderr)

        # Parse the output to find serial number
        for line in result.stdout.split('\n'):
            if 'Serial number:' in line:
                serial = line.split(':')[1].strip()
                return serial

        raise Exception("Serial number not found in YubiKey output")

    except FileNotFoundError:
        print("Error: YubiKey Manager (ykman) not found. Please install it first.")
        sys.exit(1)
    except Exception as e:
        print(f"Error: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    try:
        serial = get_yubikey_serial()
        #print(f"YubiKey Serial Number: {serial}")
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
        sys.exit(0)

# No files are created during execution
