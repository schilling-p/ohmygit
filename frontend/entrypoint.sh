#!/bin/bash
echo "Waiting for backend..."
/wait-for-it.sh ohmygit_backend:3001 --timeout=10 --strict
nginx -g 'daemon off;'