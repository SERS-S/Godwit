#!/usr/bin/env bash

if source venv/bin/activate; then
    python3 generateKey.py
else
    python3 generateKey.py
fi
