# CMAF Streamer

The project is a distributed system designed to re-encode live streams into the Common Media Application Format (CMAF). 

It has the capability to output streams in both DASH (Dynamic Adaptive Streaming over HTTP) and HLS (HTTP Live Streaming) formats with multiple Adaptation Sets and Representations. 

It uses hardware acceleration, which significantly improves the efficiency and speed of the encoding process. 

The distributed nature of the system allows for encoding tasks to be run on different GPUs and across multiple machines, thereby enhancing the scalability and performance of the system.

This project is built atop a customized version of ffmpeg
