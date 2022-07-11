# integrate-rekor

- git add README.md
- git commit --signoff
- git push origin
- ssh-keygen -C test@rekor.dev -t ed25519 -f id_ed25519 (only needed if you are creating a new set of keys)
- ssh-keygen -Y sign -n file -f id_ed25519 README.md
- cat README.md | sha256sum
- cat README.md.sig | base64 -w 0
- cat id_ed25519.pub | base64 -w 0

Rekor + sigstore-rs

New Change
