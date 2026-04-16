#!/bin/bash

# Тест определения провайдера

echo "=== Testing provider detection ==="

cd rust

# Модели которые должны определиться как Ollama
echo -e "\n1. Testing qwen2.5-coder:7b (should be Ollama)"
RUST_LOG=debug ./target/release/claw --model qwen2.5-coder:7b "hi" 2>&1 | grep -i "provider\|ollama" | head -5

echo -e "\n2. Testing llama3.2 (should be Ollama)"  
RUST_LOG=debug ./target/release/claw --model llama3.2 "hi" 2>&1 | grep -i "provider\|ollama" | head -5

echo -e "\n3. Testing vaultbox/qwen3.5-uncensored:4b (should be Ollama)"
RUST_LOG=debug ./target/release/claw --model vaultbox/qwen3.5-uncensored:4b "hi" 2>&1 | grep -i "provider\|ollama" | head -5

echo -e "\n=== Done ==="
