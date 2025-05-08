# Live CMAF Transcoder

Server capable of trancoding live streams into the Common Media Application Format (CMAF). 

It has the capability to output streams in both DASH (Dynamic Adaptive Streaming over HTTP) and HLS (HTTP Live Streaming) formats with multiple Adaptation Sets and Representations. 

It uses hardware acceleration, which significantly improves the efficiency and speed of the encoding process. 

The distributed nature of the system allows for transcoding tasks to be run on different GPUs and across multiple machines, thereby enhancing the scalability and performance of the system.

This project is built atop a customized version of ffmpeg

![Overview](https://raw.githubusercontent.com/sessystems/live-cmaf-transcoder/main/docs/images/overview.png)

## Features

- **Live Stream Re-encoding**: Convert live streams into CMAF format.
- **Multi-Format Output**: Supports both DASH and HLS streaming formats.
- **Hardware Acceleration**: Utilizes GPUs to speed up the encoding process.
- **Scalability**: Distributed system capable of running on multiple machines.


## Getting Started

Follow these instructions to set up and run the transcoder.

### Prerequisites

Ensure your system meets the following requirements: 


- **Operating System**: A Linux machine with at least 2GB of RAM and 10GB of storage. 
- **Hardware Acceleration (optional):** 
    - Intel® processor 7th generation or newer with Intel® Graphics Technology. 
    - NVidia GeForce, RTX, Quadro, or Tesla with NVENC support. 
- **Docker**: Ensure that Docker is installed on your machine. You can find the installation guide [here](https://docs.docker.com/get-docker/).
- **Docker compose**: Used to define and manage multi-container applications. You can find the installation guide [here](https://docs.docker.com/compose/install/linux/). Requires version >= v2.21.0.
- **NVIDIA Container Toolkit** (optional, for NVIDIA GPU support): If using NVIDIA GPUs, install the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) on your host machine.


### Running the Transcoder using bash Script (Option 1)

Download and execute the `run.sh` script from the [latest release](https://github.com/sessystems/live-cmaf-transcoder/releases/latest) to quickly start the transcoder on your machine.

The script will:

- Detect your hardware and enable hardware acceleration if supported  
- Pull the latest Docker image  
- Start the transcoder container  

be sure that the following tools are available on your machine to properly detect the hardware capability:
- **nvidia-smi:** Tools to check NVEnc support 

This is the easiest way to get up and running with minimal setup.

```sh
curl -L https://github.com/sessystems/live-cmaf-transcoder/releases/latest/download/run.sh | bash
```

### Running the Transcoder using Docker Compose (Option 2)

#### 1. Download compose.yaml

Get the latest `compose.yaml` file from the [latest Releases](https://github.com/sessystems/live-cmaf-transcoder/releases/latest) of the project.

```sh
curl -L -O https://github.com/sessystems/live-cmaf-transcoder/releases/latest/download/compose.yaml
```

#### 2. Pull the Latest Docker Image

Open a terminal in the same directory as your compose.yaml file and run the following command in order to get the latest version of the project:

```sh
# Requires docker compose version >= v2.21.0
# Check your version by running $> docker compose version

docker compose --profile=all pull
```

#### 3. **Set the public address of the server**

Inform the docker container about the public IP address of the server

```sh
export BASE_URL="http://$(ip route get 1 | awk '{print $7}')" # Get IP of the default interface
```

#### 4. Run **Without Hardware Acceleration Support**

To run the live-cmaf-transcoder without GPU acceleration, use the following command:

```sh
docker compose up
```

#### 5. Run **With Direct Rendering Manager (DRM) GPU Support**

To use DRM hardware acceleration, use the command below:

```sh
docker compose --profile=drm up
```

#### 6. Run **With NVidia + DRM GPU Support**

For both NVIDIA and DRM GPU support.

Ensure the NVIDIA Container Toolkit is installed on your host machine, then run:

```sh
docker compose --profile=gpu up
```

## User-Manual

The **User Manual** is available in the project wiki. You can access it [here](https://github.com/sessystems/live-cmaf-transcoder/wiki).

## Advanced Setup

### Environment Variables


The following environment variables can be used with Docker Compose to configure a transcoder instance:


| Env                    | Description             | Example        |
|------------------------|-------------------------|----------------|
| `BASE_URL`             | Public base URL of this server instance.<br>If not set, defaults to `http://localhost`. | `BASE_URL=https://www.live-cmaf-transcoder.com docker compose up`  |
| `BIND_ADDR`            | Specifies the IP address and port of the network interface where the web server should bind to.<br>If not set, defaults to `0.0.0.0:80`. | `BIND_ADDR=192.168.1.1:8080 docker compose up`  |
| `SERVER_NAME`  | Human-readable name for this server instance.<br>If not set, defaults to the machine’s hostname. | `SERVER_NAME=my-server-1 docker compose up`                                                 |
| `SERVER_UID`           | Unique identifier for this server instance.<br>Must be different for each instance sharing the same Redis database.<br>Once set, it should not be changed, doing so may result in loss of the associated configuration.<br>Defaults to the machine's Linux ID. | `SERVER_UID=1234 docker compose up`                                                         |
| `SERVER_PORT`          | Port on which the web server will be publicly exposed with Docker Compose.<br>If not set, defaults to `80`. | `SERVER_PORT=8080 docker compose up` |
| `REDIS_URL`            | Connection URL of the Redis database.<br>Use this to connect to an external Redis instance and create a cluster of transcoder servers.<br>If not set, connects to the internal Redis server running inside the docker container.                                  | `REDIS_URL=redis://192.168.1.1:6379 docker compose up`                                      |
| `REDIS_SERVICE_DISABLED` | when set to `'true'` or `'1'` it disables the internal Redis database running inside the docker container.<br>Useful when connecting to an external Redis via `REDIS_URL`. | `REDIS_SERVICE_DISABLED=true REDIS_URL=redis://192.168.1.1:6379 docker compose up` |
| `REDIS_PASSWORD`       | Password for securing the internal Redis database instance.<br>Only applies when using the internal Redis database running inside the docker container.  | `REDIS_PASSWORD=1234 docker compose up` |
| `REDIS_PORT`          | Port on which the redis server will be exposed by Docker Compose.<br>If not set, defaults to `6379`. | `REDIS_PORT=6380 docker compose up` |
| `DISABLE_TRANSCODER`  | Set to `true` to run the server without any transcoder capability. Useful to serve only the management UI. | `DISABLE_TRANSCODER=true docker compose up` |
| `DISABLE_UI`  | Set to `true` to disable the management UI. Useful for joining a cluster that already includes a UI server. | `DISABLE_UI=true docker compose up` |
| `SERVER_VERSION`  | Select the version of the server to run. <br>If not set, defaults to `latest` | `SERVER_VERSION=0.1.60 docker compose up` |

### Setting Up a Cluster of Transcoders

You can group multiple transcoder instances together to form a cluster. This allows multiple servers to share workload and state through a common Redis database.

Below is a step-by-step example of how to configure two servers to work as a cluster.

#### Example Setup

| Server   | Type      | IP Address     |
|----------|-----------|----------------|
| Server 1 | Primary node (transcoder + redis + management UI)  | `192.168.1.1`  |
| Server 2 | Joining node (transcoder only)  | `192.168.1.2`  |

#### Step 1 — Start the Primary Instance on Server 1

Run the following command on **Server 1** to start it:

```bash
BASE_URL=http://192.168.1.1 \
SERVER_NAME="Server 1" \
SERVER_UID=1 \
REDIS_PASSWORD=1234 \
docker compose --profile=gpu up
```

#### Step 2 — Join Server 2 to the Cluster

Run the following command on **Server 2** to connect it to the cluster using the Redis instance on Server 1:

```bash
BASE_URL=http://192.168.1.2 \
SERVER_NAME="Server 2" \
SERVER_UID=2 \
REDIS_URL=redis://:1234@192.168.1.1:6379 \
REDIS_SERVICE_DISABLED=true \
DISABLE_UI=true \
docker compose --profile=gpu up
```

`REDIS_URL` points to the external Redis instance on **Server 1**.  
`REDIS_SERVICE_DISABLED=true` disables the internal Redis server on **Server 2**.  
`DISABLE_UI=true` disables the management UI on **Server 2**.  

#### Step 3 — Access the Web UI

Open the following URL in your browser to access the web interface:
```http://192.168.1.1```

You should see both transcoder instances listed as active in the cluster dashboard, as shown in the image below:

![Overview](https://raw.githubusercontent.com/sessystems/live-cmaf-transcoder/main/docs/images/overview_cluster.png)

## Contributing

We welcome contributions! Please read our [contributing guidelines](CONTRIBUTING.md) before submitting a pull request.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
