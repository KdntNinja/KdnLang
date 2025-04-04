import sys
import os

# Cross-platform password input with '*' masking
if os.name == 'nt':
    import msvcrt
else:
    import tty
    import termios

def input_password(prompt="Password: "):
    print(prompt, end='', flush=True)
    password = ''
    
    if os.name == 'nt':
        while True:
            ch = msvcrt.getch()
            if ch in {b'\r', b'\n'}:
                break
            elif ch == b'\x08':  # Backspace
                if len(password) > 0:
                    password = password[:-1]
                    sys.stdout.write('\b \b')
            else:
                try:
                    char = ch.decode('utf-8')
                    password += char
                    sys.stdout.write('*')
                except:
                    continue
    else:
        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd)
        try:
            tty.setraw(fd)
            while True:
                ch = sys.stdin.read(1)
                if ch in {'\r', '\n'}:
                    break
                elif ch == '\x7f':  # Backspace
                    if len(password) > 0:
                        password = password[:-1]
                        sys.stdout.write('\b \b')
                else:
                    password += ch
                    sys.stdout.write('*')
        finally:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)

    print()
    return password

# Password manager core
def save_password():
    service = input("Service name: ")
    password = input_password("Password: ")

    with open("passwords.txt", "a") as f:
        f.write(f"{service}: {password}\n")

    print("Saved.")

def get_password():
    service = input("Service name to find: ")

    try:
        with open("passwords.txt", "r") as f:
            for line in f:
                if line.startswith(service + ":"):
                    print("Password found:", line.split(": ", 1)[1].strip())
                    return
    except FileNotFoundError:
        print("No passwords saved yet.")
        return

    print("Password not found.")

def main():
    print("1. Save password")
    print("2. Get password")
    choice = input("Choice: ")

    if choice == "1":
        save_password()
    elif choice == "2":
        get_password()
    else:
        print("Invalid choice.")

if __name__ == "__main__":
    main()
