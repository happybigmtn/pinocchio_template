#!/bin/bash
echo "=== DEBUG INFO ==="
echo "PWD: $PWD"
echo "ORIGINAL_PWD: $ORIGINAL_PWD"  
echo "INIT_CWD: $INIT_CWD"
echo "npm_config_pwd: $npm_config_pwd"
echo "OLDPWD: $OLDPWD"
echo "All env vars with CWD/PWD:"
env | grep -i "pwd\|cwd" | head -10
echo "=================="
