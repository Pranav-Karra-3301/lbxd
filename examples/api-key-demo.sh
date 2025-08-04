#!/bin/bash

# Example script demonstrating API key configuration for lbxd

echo "================================================"
echo "lbxd API Key Configuration Example"
echo "================================================"
echo

echo "1. Default behavior (uses built-in API keys):"
echo "   lbxd movie \"Inception\""
echo "   -> Uses default TMDB and OMDB API keys"
echo

echo "2. Using custom TMDB API key:"
echo "   export TMDB_API_KEY=\"your_tmdb_key_here\""
echo "   lbxd movie \"Inception\""
echo "   -> Uses your TMDB key, default OMDB key"
echo

echo "3. Using custom OMDB API key:"
echo "   export OMDB_API_KEY=\"your_omdb_key_here\""
echo "   lbxd recent username"
echo "   -> Uses default TMDB key, your OMDB key"
echo

echo "4. Using both custom API keys:"
echo "   export TMDB_API_KEY=\"your_tmdb_key_here\""
echo "   export OMDB_API_KEY=\"your_omdb_key_here\""
echo "   lbxd browse username"
echo "   -> Uses both custom keys"
echo

echo "5. Temporary override for single command:"
echo "   TMDB_API_KEY=\"temp_key\" lbxd movie \"Dune\""
echo "   -> Uses temporary key only for this command"
echo

echo "================================================"
echo "No Xcode, keychain, or complex setup required!"
echo "Works immediately after installation."
echo "================================================"

# Optional: Demonstrate current environment
echo
echo "Current API key configuration:"
echo "TMDB_API_KEY: ${TMDB_API_KEY:-[using default]}"
echo "OMDB_API_KEY: ${OMDB_API_KEY:-[using default]}"
