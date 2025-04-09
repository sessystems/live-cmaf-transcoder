# Live CMAF Transcoder

Distributed system designed to re-encode live streams into the Common Media Application Format (CMAF). 

It has the capability to output streams in both DASH (Dynamic Adaptive Streaming over HTTP) and HLS (HTTP Live Streaming) formats with multiple Adaptation Sets and Representations. 

It uses hardware acceleration, which significantly improves the efficiency and speed of the encoding process. 

The distributed nature of the system allows for encoding tasks to be run on different GPUs and across multiple machines, thereby enhancing the scalability and performance of the system.

This project is built atop a customized version of ffmpeg

![Overview](https://raw.githubusercontent.com/sessystems/live-cmaf-transcoder/main/docs/images/overview.png)

## Features

- **Live Stream Re-encoding**: Convert live streams into CMAF format.
- **Multi-Format Output**: Supports both DASH and HLS streaming formats.
- **Hardware Acceleration**: Utilizes GPUs to speed up the encoding process.
- **Scalability**: Distributed system capable of running on multiple machines.

## Installation

To install the necessary dependencies, run:
```sh
sudo apt-get update
sudo apt-get install -y ffmpeg
```

## Usage

To start the CMAF Streamer, use the following command:
```sh
./start-streamer.sh
```

## Contributing

We welcome contributions! Please read our [contributing guidelines](CONTRIBUTING.md) before submitting a pull request.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
