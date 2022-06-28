# integrate-rekor

- git add README.md
- git commit --signoff
- git push origin
- ssh-keygen -Y sign -n file -f id_ed25519 README.md
- cat README.md | sha256sum
- cat README.md.sig | base64 -w 0

Rekor + sigstore-rs

New Change
