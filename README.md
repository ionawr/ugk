# ugk

a program to **u**nlock **g**nome-**k**eyring

## usage

ugk generates and manages gpg encrypted file that contains the keyring name -
password pairs

secretfile path configuration precendence:

- environment variable `UGK_SECRETFILE_PATH`
- flag `--secretfile`
- `$XDG_CONFIG_HOME/ugk/secret.toml`
- `$HOME/.config/ugk/secret.toml`

to initialise the secretfile: `ugk init`

to add a new keyring - password pair:
`ugk add -r <RECIPIENT> -k <KEYRING_NAME>` you will be prompted to enter your
password

to remove a registerd keyring - password pair:
`ugk rm -r <RECIPIENT> -k <KEYRING_NAME>`

TODO: actually using ugk to unlock the keyrings

## todo

- [X] ~generate and encrypt secret file (`create_secret_file.sh`)~ -> do this
      with gpg directly instead
- [X] manage keyring password pairs (KeyringCredential)
- [X] decrypt secret file (first half of `unlock_keyring.sh`)
  - args: pubkey id (recipient), secret file location (encrypted with sops)
- [ ] use D-bus to unlock gnome keyring (second half of `unlock_keyring.sh`, plus
  the cpp program)
  - args: secret file location
