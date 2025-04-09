<template>
  <v-container fluid>
    <v-row>
      <v-col>
        <v-card style="min-height: 100%;">
          <v-card-title>MPEG-DASH</v-card-title>
          <v-card-text>
            <v-row>
              <v-col>
                <video
                  ref="player"
                  class="video-js vjs-16-9"
                  controls
                  preload="auto"
                  data-setup="{}"
                />
              </v-col>
            </v-row>
            <v-row>
              <v-col>
                <v-tooltip
                  text="Copy to Clipboard"
                  location="bottom"
                >
                  <template #activator="{ props }">
                    <a
                      v-bind="props"
                      class="text-primary cursor-pointer"
                      target="_blank"
                      @click="utils.copyToClipboard(dash_url)"
                    >
                      {{ dash_url }}
                    </a>
                  </template>
                </v-tooltip>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col>
        <v-card style="min-height: 100%;">  
          <v-card-title>HLS</v-card-title>
          <v-card-text>
            <v-row v-if="config?.enableHls !== true">
              <v-col>Disabled</v-col>
            </v-row>
            <span v-if="config?.enableHls !== true">Disabled</span>
            <v-row v-if="config?.enableHls === true">
              <v-col>
                <video
                  ref="player_hls"
                  class="video-js vjs-16-9"
                  controls
                  preload="auto"
                  data-setup="{}"
                />
              </v-col>
            </v-row>
            <v-row v-if="config?.enableHls === true">
              <v-col>
                <v-tooltip
                  text="Open with HLS.js Test App"
                  location="bottom"
                >
                  <template #activator="{ props }">
                    <a
                      v-bind="props"
                      class="text-primary cursor-pointer"
                      target="_blank"
                      @click="utils.copyToClipboard(hls_url)"
                    >
                      {{ hls_url }}
                    </a>
                  </template>
                </v-tooltip>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>
<script setup lang="ts">
import videojs from 'video.js';
import 'jb-videojs-hls-quality-selector'
const ffapi = useFFApi();
const player = ref();
const player_hls = ref();
const utils = useUtils();

const video_js_dash = ref();
const video_js_hls = ref();

const _props = defineProps<{
  configUid: string;
}>();

const config = computed(() => ffapi.getConfig(_props.configUid));
const dash_url = computed(() => {
  if (config.value === undefined) {
    return "";
  }
  return utils.get_dash_url(config.value);
});
const hls_url = computed(() => {
  if (config.value === undefined) {
    return "";
  }
  return utils.get_hls_url(config.value);
});

onMounted(() => {
  video_js_dash.value = videojs(player.value, {
    controls: true,
    autoplay: 'muted',
    preload: "auto",
    fluid: true,
    sources: [{
      src: dash_url.value,
      type: "application/dash+xml"
    }]
  });

  video_js_dash.value.hlsQualitySelector();

  if (config.value?.enableHls === true) {
    video_js_hls.value = videojs(player_hls.value, {
      controls: true,
      autoplay: 'muted',
      preload: "auto",
      fluid: true,
      sources: [{
        src: hls_url.value,
        type: "application/vnd.apple.mpegurl"
      }]
    });

    video_js_hls.value.hlsQualitySelector();
  }

});
onBeforeUnmount(() => {

  if (video_js_dash.value !== undefined) {
    video_js_dash.value.dispose();
  }

  if (video_js_hls.value !== undefined) {
    video_js_hls.value.dispose();
  }

});

</script>