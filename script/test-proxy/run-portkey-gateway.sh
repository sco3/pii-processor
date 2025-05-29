docker run --name=portkey-gateway --network=bridge --workdir=/app -p 8787:8787 --runtime=runc --detach=true portkeyai/gateway:latest run start:node
