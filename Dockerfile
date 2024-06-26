FROM kquirapas/solana-builder:v0.1.0

WORKDIR /app
COPY . .

CMD ["cargo", "build-sbf"]
