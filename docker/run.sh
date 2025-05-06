#!/bin/bash
set -euo pipefail

GREEN="\033[0;32m"
RED="\033[0;31m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
NC="\033[0m" # No Color

SERVER_VERSION="latest"

info()    { local indent="${2:-}"; echo -e "${indent}${BLUE}[+]${NC} $1"; }
success() { local indent="${2:-}"; echo -e "${indent}${GREEN}✔${NC} $1"; }
warn()    { local indent="${2:-}"; echo -e "${indent}${YELLOW}⚠️ ${NC} $1"; }
error()   { local indent="${2:-}"; echo -e "${indent}${RED}❌${NC} $1"; }

has_drm() {
    local indent=" "
    local render_devices=(/dev/dri/renderD*)
    if [ ${#render_devices[@]} -gt 0 ]; then
        success "Found render device(s): ${render_devices[*]}" "$indent"
        return 0
    else
        warn "No /dev/dri/renderD* devices found." "$indent"
        return 1
    fi
}
has_nvidia() {

    local indent=" "
    if ! command -v nvidia-smi &> /dev/null; then
        error "Missing 'nvidia-smi' tool. Please install it to detect NVIDIA GPUs." "$indent"
        return 1
    fi

    local nvidia_info=$(nvidia-smi --query-gpu=index,name,uuid --format=csv,noheader)
    if [[ -n "$nvidia_info" ]]; then
        success "NVIDIA GPU detected." "$indent"
    else
        warn "No NVIDIA GPU detected." "$indent"
        return 1
    fi

    if ! command -v nvidia-container-toolkit &> /dev/null; then
        warn "NVIDIA Container Toolkit is not installed. NVIDIA GPU acceleration may not work inside Docker." "$indent"
        return 1
    fi
    success "NVIDIA Container Toolkit detected." "$indent"
    return 0
}

has_docker() {
    local indent=" "
    info "Checking Docker installation..."
    if ! command -v docker &> /dev/null; then
        error "Docker is not installed. Please install Docker and try again." "$indent"
        return 1
    fi

    if ! docker info &> /dev/null; then
        error "Docker is not running or current user lacks access. Please start Docker or fix permissions." "$indent"
        return 1
    fi

    
    if ! docker compose version &> /dev/null; then
        error "Docker compose is not install. Please install Docker compose and try again." "$indent"
        return 1
    fi

    local docker_compose_version=$(docker compose version --short)
    if [ "$(printf '%s\n' "2.21.0" "$docker_compose_version" | sort -V | head -n1)" != "2.21.0" ]; then
        error "Docker Compose version must be >= 2.21.0. Found: $docker_compose_version"
        return 1
    fi
    success "docker compose version: $docker_compose_version >= 2.21.0" "$indent"

    success "Docker is available" "$indent"
    return 0
}

download_latest_compose() {
    info "Downloading latest version of the live-cmaf-transcoder compose.yaml..."
    if ! curl -fsSL -o compose.yaml https://github.com/sessystems/live-cmaf-transcoder/releases/latest/download/compose.yaml; then
            error "Failed to download compose.yaml"
            exit 1
    fi
    success "compose.yaml Downloaded " " "
}

docker_load_latest() {
    info "Pulling the latest container images..."
    download_latest_compose
    docker compose --profile=all pull  || {
        error "Failed to pull latest image."
        exit 1
    }
}

docker_load_from_file() {
    info "Loading docker image from file..."

    [[ -f "${DOCKER_IMAGE_FILE}" ]] || {
        error "${DOCKER_IMAGE_FILE} file not found."
        exit 1
    }

    success "${DOCKER_IMAGE_FILE} file found" " "
    docker load < <(xz -dc ${DOCKER_IMAGE_FILE}) || {
        error "Failed to load docker image."
        exit 1
    }

    if [[ -f "compose.yaml" ]]; then
        success "compose.yaml found" " "
    else
        download_latest_compose
    fi
}

print_help() {
  echo "Usage: $0 [OPTIONS]"
  echo ""
  echo "Options:"
  echo "  --version=VERSION   Specify the version to use (e.g. latest, 1.0.60)"
  echo "  --help, -?          Show this help message"
}

#==============================================================#

while getopts ":v:o:c:h?-:" opt; do
  case "$opt" in
    v) SERVER_VERSION="$OPTARG" ;;
    h) print_help; exit 0 ;;
    -)  # Handle long options
        case "$OPTARG" in
          version=*) SERVER_VERSION="${OPTARG#*=}" ;;
          help) print_help; exit 0 ;;
          *) echo "Unknown option: --$OPTARG" >&2; print_help; exit 1 ;;
        esac ;;
    \?) echo "Invalid option: -$OPTARG" >&2; print_help; exit 1 ;;
    :) echo "Option -$OPTARG requires an argument." >&2; print_help; exit 1 ;;
  esac
done

echo -e "${BLUE}╔═══════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        🚀 Starting live-cmaf-transcoder       ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════╝${NC}"
echo ""

has_docker || {
    error "Docker is not installed or not running. Please install Docker and try again."
    exit 1
}

info "Checking GPU support..."
profile="cpu"
if has_drm; then
    profile="drm"
fi
if has_nvidia; then
    profile="gpu"
fi
success "Selecting profile: $profile" " "

success "Selecting server version: ${SERVER_VERSION}" " "
export SERVER_VERSION="${SERVER_VERSION}"
DOCKER_IMAGE_FILE="sessystems-live-cmaf-transcoder-${SERVER_VERSION}.tar.xz"

if [[ $SERVER_VERSION == "latest" ]]; then
    docker_load_latest
elif [[ -f "${DOCKER_IMAGE_FILE}" ]]; then
    docker_load_from_file
else 
    download_latest_compose
fi

info "Stopping and removing any existing containers..."
docker compose --profile=all down || true

info "Starting containers using profile '${profile}'..."
export BASE_URL="http://$(ip route get 1 | awk '{print $7}')"
docker compose --profile=${profile} up -d || {
    error "Failed to start Docker containers."
    exit 1
}

echo ""
echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "                                           ${GREEN}\033[1m🎉 All done!\033[0m${NC}"
echo -e "  🌐  Web App:     ${GREEN}${BASE_URL}${NC}"
echo -e "  🧹  Stop:        ${YELLOW}docker compose --profile=all down${NC}"
echo -e "  🚀  Start again: ${YELLOW}BASE_URL=\"${BASE_URL}\" SERVER_VERSION=\"${SERVER_VERSION}\" docker compose --profile=${profile} up -d${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════════════════════════════════════════════════╝${NC}"