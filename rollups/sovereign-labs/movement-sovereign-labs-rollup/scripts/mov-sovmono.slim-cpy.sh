#!/bin/bash -e
RUSTFLAGS="--cfg tokio_unstable" cargo build -p rollup --target x86_64-unknown-linux-gnu

export DOCKER_BUILDKIT=1
aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin public.ecr.aws
docker build --platform linux/amd64 --ssh default -t sov-monovm:latest -f mov-sovmono.slim-cpy.dockerfile .
docker tag sov-monovm:latest public.ecr.aws/c4i6k4r8/sov-monovm:latest
docker push public.ecr.aws/c4i6k4r8/sov-monovm:latest