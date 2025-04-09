FROM live-cmaf-transcoder
RUN apt-get update && apt-get install -y redis
WORKDIR /opt
COPY ./redis.sh /opt/nvidia/entrypoint.d/94redis.sh
COPY ./demo.sh /opt/nvidia/entrypoint.d/95demo.sh
WORKDIR /demo
COPY ./video.mp4 /demo/video.mp4
RUN chmod +x /opt/nvidia/entrypoint.d/95demo.sh
ENTRYPOINT ["/opt/nvidia/nvidia_entrypoint.sh", "live-cmaf-transcoder"]