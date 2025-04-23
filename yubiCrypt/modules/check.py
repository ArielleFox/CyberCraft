import subprocess
import sys

def check_yubikey_slots():
    try:
        # Run ykman info command and capture output
        result = subprocess.run(['ykman', 'otp', 'info'],
                              capture_output=True,
                              text=True)

        if result.returncode != 0:
            raise Exception("Error running ykman: " + result.stderr)

        slots_status = {
            'slot1': False,
            'slot2': False
        }

        # Parse the output to check slots
        for line in result.stdout.split('\n'):
            if 'Slot 1:' in line and 'empty' not in line.lower():
                slots_status['slot1'] = True
            if 'Slot 2:' in line and 'empty' not in line.lower():
                slots_status['slot2'] = True

        return slots_status

    except FileNotFoundError:
        print("Error: YubiKey Manager (ykman) not found. Please install it first.")
        sys.exit(1)
    except Exception as e:
        print(f"Error: {str(e)}")
        sys.exit(1)

def print_slot_status(slots_status):
    print("\nYubiKey Slot Status:")
    print("-" * 20)
    print(f"Slot 1: {'Configured' if slots_status['slot1'] else 'Empty'}")
    print(f"Slot 2: {'Configured' if slots_status['slot2'] else 'Empty'}")

if __name__ == "__main__":
    try:
        print("Checking YubiKey slots...")
        slots_status = check_yubikey_slots()
        print_slot_status(slots_status)
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
        sys.exit(0)

# No files are created during execution
