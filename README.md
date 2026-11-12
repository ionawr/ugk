# ugk

unlock gnome-keyring

## todo

a program that can:

- [ ] ~generate and encrypt secret file (`create_secret_file.sh`)~ -> do this
      with gpg directly instead
- [ ] manage keyring password pairs (KeyringCredential)
- [ ] decrypt secret file (first half of `unlock_keyring.sh`)
  - args: pubkey id (recipient), secret file location (encrypted with sops)
  - somehow run sops to decrypt the file? (shell script?)
- [ ] use D-bus to unlock gnome keyring (second half of `unlock_keyring.sh`, plus
  the cpp program)
  - args: secret file location
