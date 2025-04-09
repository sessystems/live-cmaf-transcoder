FROM nvidia-runtime
ARG DEBIAN_FRONTEND="noninteractive"

RUN apt-get update && apt-get install -y \
    libx264-163 \
    libx265-199 \
    libxml2 \
    libva2 \
    libfreetype6 \
    libfontconfig1 \
    libharfbuzz0b \
    libfribidi0 \
    libfdk-aac2 \
    libsrt1.4-gnutls \
    libgnutls30 \
    libdrm2 \
    libva-drm2 \
    libnuma1 \
    libmp3lame0 \
    vainfo \
    apt-utils \
    intel-media-va-driver-non-free \ 
    redis
 
COPY --from=backend /app/target/release/live-cmaf-transcoder /usr/local/bin
COPY --from=ffmpeg /app/ffmpeg/ffmpeg /usr/local/bin
COPY --from=ffmpeg /app/ffmpeg/ffprobe /usr/local/bin
COPY ./docker/redis.sh /opt/nvidia/entrypoint.d/94redis.sh

RUN  echo "**** clean up ****" && \
    rm -rf \
    /var/lib/apt/lists/* \
    /var/tmp/*

ENTRYPOINT ["/opt/nvidia/nvidia_entrypoint.sh", "live-cmaf-transcoder", "--ffmpeg", "/usr/local/bin/ffmpeg"]
    
