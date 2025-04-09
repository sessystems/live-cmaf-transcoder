fftools/ffmpeg.c : Improve ffmpeg stat for live by calculating fps average over a time window
fftools/ffmpeg_mux.c, libavfilter/vf_hwdownload.c, libavformat/rtsp.c : Fix, live encoding is stopping after dts discontinuity
libavformat/dashenc.c : New option to apply an offset to Availabiliy Start Time, re-sync DASH Clock with encoder clock if a drift between the clocks is detected