# ugk

unlock gnome-keyring

## todo

a program that can:

- generate and encrypt secret file (`create_secret_file.sh`)
  - args: pubkey id (recipient), secret file location
  - probably set permissions to a safer one (like how ssh does)
- decrypt secret file (first half of `unlock_keyring.sh`)
  - args: pubkey id (recipient), secret file location
- use D-bus to unlock gnome keyring (second half of `unlock_keyring.sh`, plus
  the cpp program)
  - args: secret file location
