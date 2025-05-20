#!/bin/sh

#echo "Waiting for backend DNS resolution..."

#until getent hosts ohmygit_backend > /dev/null; do
 # echo "Backend hostname not resolvable yet, retrying..."
  #sleep 1
#done

#echo "Backend hostname resolved!"

/wait-for-it.sh ohmygit_backend:3001 --timeout=10 --strict -- \
  sh -c "echo 'Backend is up, starting nginx...' && nginx -g 'daemon off;'"
