#!/bin/env python3.14
import ast
import os

def read_identities_file(filename="identities.dict"):
    """Reads the identities.dict file and parses it into a dictionary."""
    try:
        with open(filename, "r") as f:
            content = f.read()
            return ast.literal_eval(content)  # Safely parse the dictionary
    except Exception as e:
        print(f"Error reading {filename}: {e}")
        return {}

def format_identity(slot, data):
    """Formats a single identity entry into the desired format."""
    lines = data.split("\n")
    formatted = f"# Slot: {slot}\n"
    for line in lines:
        if line.strip():  # Skip empty lines
            formatted += f"{line.strip()}\n"
    return formatted

def save_formatted_identities(identities, output_filename="formatted_identities.txt"):
    """Formats and saves all identities into a single file."""
    try:
        # Format all identities
        formatted_data = ""
        for slot, data in sorted(identities.items()):
            formatted_data += format_identity(slot, data) + "\n"

        # Check if the file already exists and has the same content
        if os.path.exists(output_filename):
            with open(output_filename, "r") as f:
                existing_content = f.read()
                if existing_content == formatted_data:
                    print(f"No changes detected. {output_filename} remains unchanged.")
                    return

        # Write the formatted data to the file
        with open(output_filename, "w") as f:
            f.write(formatted_data)
        #print(f"Formatted identities saved to {output_filename}")

    except Exception as e:
        print(f"Error writing to {output_filename}: {e}")

def main():
    # Read the identities.dict file
    identities = read_identities_file("identities.dict")
    if not identities:
        print("No identities found or failed to read the file.")
        return

    # Save the formatted identities to a new file
    save_formatted_identities(identities, "formatted_identities.txt")

if __name__ == "__main__":
    main()
