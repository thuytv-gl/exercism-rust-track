#!bash

nvim \
  -c ":nmap <M-s> :!exercism submit<CR>" \
  -c ":nmap <M-r> :!cargo test -- --ignored<CR>" $@
