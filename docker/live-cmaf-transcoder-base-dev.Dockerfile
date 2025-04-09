FROM nvidia-dev
ARG DEBIAN_FRONTEND="noninteractive"
ARG NV_CODEC_HEADERS_VERSION
RUN apt-get update && apt-get install -y yasm  \
    pkg-config \
    unzip \
    git \
    libx264-dev \
    libx265-dev \
    libxml2-dev \
    libva-dev \
    libfreetype6-dev \
    libfontconfig1-dev \
    libharfbuzz-dev \
    libfribidi-dev \
    libfdk-aac-dev \
    libsrt-gnutls-dev \
    libmp3lame-dev \
    libnuma-dev \
    libgnutls28-dev \
    build-essential \
    curl
    
WORKDIR /app/nv
RUN git clone --branch ${NV_CODEC_HEADERS_VERSION} https://github.com/FFmpeg/nv-codec-headers.git . 
RUN make install