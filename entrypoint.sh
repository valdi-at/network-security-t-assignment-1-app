#!/bin/sh
set -e

# Map SITE_ADDR → LEPTOS_SITE_ADDR (with fallback)
export LEPTOS_SITE_ADDR="0.0.0.0:${PORT:-3000}"

exec "$@"