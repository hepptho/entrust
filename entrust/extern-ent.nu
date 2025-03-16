def "nu-complete nothing" [] { [] }
def "nu-complete ent backend" [] { ["age", "gpg" ] }
def "nu-complete ent existing-file" [] {
  cd $env.ENT_STORE
  glob --no-dir **/[!.]* | path relative-to $env.ENT_STORE | str replace --all '\' '/'
}
def "nu-complete ent existing-file-or-dir" [] {
  cd $env.ENT_STORE
  glob **/[!.]* | path relative-to $env.ENT_STORE | str replace --all '\' '/'
}
def "nu-complete ent generate type" [] { ["phrase", "word"] }

# Add a new password
export extern "ent add" [
  key?: string@"nu-complete nothing"               # The key under which to store the encrypted file
  --backend (-b): string@"nu-complete ent backend" # Choose gpg or age for encryption
  --no-git                                         # Do not add the new file to git
]

# Autotype into the previously active window
export extern "ent autotype" [
  segments: string@"nu-complete nothing" # One or more keys, separated by a colon, and optionally {tab} or {enter}
]

# Change an existing password
export extern "ent edit" [
  key?: string@"nu-complete ent existing-file"      # The key of the password to edit
  --cleartext (-c)                                  # Edit the password in cleartext
  --backend (-b): string@"nu-complete ent backend"  # Choose gpg or age for re-encryption
]

# Generate a random password
export extern "ent generate" [
  type?: string@"nu-complete ent generate type"
  --clipboard (-c)                                 # Copy the generated password to the clipboard
  --store (-s): string@"nu-complete nothing"       # Encrypt and store the generated password under the given key
  --length (-l): int                               # Length of the password (default: 7 words for type phrase; 20 characters for type word)
  --sep: string@"nu-complete nothing"              # Word separator for type phrase
  --backend (-b): string@"nu-complete ent backend" # Choose gpg or age for encryption
  --no-anim (-n)                                   # Skip the flashy animation when printing to stdout
  --no-git                                         # Do not add the file to the git repository if one exists (only effective with --store)
]

# Decrypt a password
export extern "ent get" [
  key?: string@"nu-complete ent existing-file" # The key of the password to decrypt
  --clipboard (-c)                             # Copy the password to the clipboard
]

# Move a password to another location in the store
export extern "ent mv" [
  from?: string@"nu-complete ent existing-file-or-dir"
  to?: string
]

# Move a password to another location in the store
export extern "ent rm" [
  from?: string@"nu-complete ent existing-file-or-dir" # The key to delete
  --recurse (-r)                                       # Enable deleting directories
]
