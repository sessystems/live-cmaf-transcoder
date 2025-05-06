
now = timestamp()
authors = "Yannick Poirier <yannick.poirier@ses.com>; Xavier Scholtes <xavier.scholtes@ses.com>"
url = "https://github.com/sessystems/live-cmaf-transcoder"
description = "Re-encode live streams into the Common Media Application Format (CMAF).\nOutput streams in both DASH and HLS.\nIt uses hardware acceleration (GPUs)."

target "_common_labels" {
    annotations = [
    "index,manifest:org.opencontainers.image.created=${now}",
    "index,manifest:org.opencontainers.image.url=${url}",
    "index,manifest:org.opencontainers.image.source=${url}",
    "index,manifest:org.opencontainers.image.vendor=${authors}",
    "index,manifest:org.opencontainers.image.title=live-cmaf-transcoder",
    "index,manifest:org.opencontainers.image.description=${description}",
    "index,manifest:org.opencontainers.image.documentation=${url}",
    "index,manifest:org.opencontainers.image.authors=${authors}",
    "index,manifest:org.opencontainers.image.licenses=Apache-2.0",
  ]
  labels = {
    "org.opencontainers.image.created" = "${now}",
    "org.opencontainers.image.url" = "${url}",
    "org.opencontainers.image.source" = "${url}",
    "org.opencontainers.image.vendor" = "${authors}",
    "org.opencontainers.image.title" = "live-cmaf-transcoder",
    "org.opencontainers.image.description" = "${description}",
    "org.opencontainers.image.documentation" = "${url}",
    "org.opencontainers.image.authors" = "${authors}",
    "org.opencontainers.image.licenses" = "Apache-2.0"
  }
}

variable "REGISTRY_DOCKER" {
  default = "docker.io"
}

variable "REGISTRY_GITHUB" {
  default = "ghcr.io"
}

variable "ORGREPOS" {
  default = "sessystems"
}

variable "VERSION" {
  description = "The version tag for the Docker image"
  default = "" # Default is empty, we will set it in the workflow
}

group "default" {
    targets = [
        "live-cmaf-transcoder-nv-11-1-ffmpeg-7-0",
    ]
}

group "all" {
    targets = [
        "live-cmaf-transcoder",
    ]
}

target "live-cmaf-transcoder-base-dev" {
    dockerfile = "docker/live-cmaf-transcoder-base-dev.Dockerfile"
    context = "."
    name="live-cmaf-transcoder-base-dev-nv-${item.nv-tag}"
    matrix = {
        item = [
            {
                nv-tag="12-0"
                nvidia-dev = "nvidia/cuda:12.4.1-devel-ubuntu22.04"
                nv-codec-headers = "sdk/12.0"
            },
            {
                nv-tag="11-1"
                nvidia-dev = "nvidia/cuda:12.4.1-devel-ubuntu22.04"
                nv-codec-headers = "sdk/11.1"
            }
        ]
    }
    contexts = {
        nvidia-dev = "docker-image://${item.nvidia-dev}"
    }
    args = {
        NV_CODEC_HEADERS_VERSION = "${item.nv-codec-headers}"
    }
    
}

target "ffmpeg" {
    dockerfile="docker/ffmpeg-${item.variant}.Dockerfile"
    name="ffmpeg-${item.variant}-nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}"
    contexts = { 
        cmaf-dev = "target:live-cmaf-transcoder-base-dev-nv-${item.nv-tag}"
    }
    matrix = {
        item = [
            {
                variant = "gpl",
                ffmpeg-tag="7-0"
                nv-tag="12-0"
            },
            {
                variant = "non-free",
                ffmpeg-tag="7-0"
                nv-tag="12-0"
            },
            {
                variant = "gpl",
                ffmpeg-tag="7-0"
                nv-tag="11-1"
            },
            {
                variant = "non-free",
                ffmpeg-tag="7-0"
                nv-tag="11-1"
            }
        ]
    }
    args = {
        FFMEPG_BRANCH = replace("${item.ffmpeg-tag}", "-", ".")
    }
    
}

target "cmaf-frontend" {
    dockerfile="docker/frontend.Dockerfile"
}

target "backend" {
    dockerfile="docker/backend.Dockerfile"
    name="backend-nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}"
    contexts = {
        cmaf-dev = "target:live-cmaf-transcoder-base-dev-nv-${item.nv-tag}"
        cmaf-frontend = "target:cmaf-frontend"
    }
    matrix = {
        item = [
            {
                nv-tag="12-0"
                ffmpeg-tag="7-0"
            },
            {
                nv-tag="11-1"
                ffmpeg-tag="7-0"
            }
        ]
    }
    
}

target "live-cmaf-transcoder" {
    inherits = ["_common_labels"]
    dockerfile="docker/live-cmaf-transcoder.Dockerfile"
    name="live-cmaf-transcoder-nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}"
    contexts = {
        backend = "target:backend-nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}"
        ffmpeg-gpl = "target:ffmpeg-gpl-nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}"
        ffmpeg-non-free = "target:ffmpeg-non-free-nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}"
        nvidia-runtime = "docker-image://${item.nvidia-runtime}"
    }
    tags = [
        equal("latest","${item.tag}") ? "${REGISTRY_GITHUB}/${ORGREPOS}/live-cmaf-transcoder:${item.tag}": "",
        equal("latest","${item.tag}") ? "${REGISTRY_DOCKER}/${ORGREPOS}/live-cmaf-transcoder:${item.tag}": "",
        "${REGISTRY_GITHUB}/${ORGREPOS}/live-cmaf-transcoder:nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}",
        "${REGISTRY_DOCKER}/${ORGREPOS}/live-cmaf-transcoder:nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}",
        notequal("",VERSION) ? "${REGISTRY_GITHUB}/${ORGREPOS}/live-cmaf-transcoder:nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}-v${VERSION}": "",
        notequal("",VERSION) ? "${REGISTRY_DOCKER}/${ORGREPOS}/live-cmaf-transcoder:nv-${item.nv-tag}-ffmpeg-${item.ffmpeg-tag}-v${VERSION}": "",
        notequal("",VERSION) && equal("latest","${item.tag}") ? "${REGISTRY_GITHUB}/${ORGREPOS}/live-cmaf-transcoder:${VERSION}": "",
        notequal("",VERSION) && equal("latest","${item.tag}") ? "${REGISTRY_DOCKER}/${ORGREPOS}/live-cmaf-transcoder:${VERSION}": "",
    ] 
    matrix = {
        item = [
            {
                nv-tag="12-0"
                ffmpeg-tag="7-0"
                nvidia-runtime = "nvidia/cuda:12.4.1-runtime-ubuntu22.04"
                tag = "latest"
            },
            {
                nv-tag="11-1"
                ffmpeg-tag="7-0"
                nvidia-runtime = "nvidia/cuda:12.4.1-runtime-ubuntu22.04"
                tag = ""
            }
        ]
    }
    
}


