<template>
  <v-container fluid>
    <v-row>
      <v-col>
        <h4>CMAF Live Stream Transcoder</h4>
      </v-col>
    </v-row>

    <v-row>
      <v-col>
        <p>The CMAF Live Stream Transcoder is designed for real-time transcoding of live video sources into CMAF formats (MPEG-DASH, HLS).</p> 
      </v-col>
    </v-row>

    <v-row>
      <v-col>
        <ff-create-encoder :server-uid="serverUid" />
      </v-col>
    </v-row>
    
    <v-table>
      <thead>
        <tr>
          <th class="text-left">
            Name
          </th>
          <th class="text-left">
            Monitor
          </th>
          <th class="text-left">
            Status
          </th>
          <th class="text-left">
            Acceleration
          </th>
          <th
            class="text-left" 
            style="min-width:80px"
          >
            <div>NB Video</div><div>Representations</div>
          </th>
          <th
            class="text-left" 
            style="min-width:80px"
          >
            <div>Encoding</div><div>Speed</div>
          </th>
          <th
            class="text-left"
            style="min-width:95px"
          >
            FPS
          </th>
          <th class="text-left">
            <div>Dropped / Duplicated</div>
            <div>Frames</div>
          </th>
          <th
            class="text-left"
            style="min-width:85px"
          >
            <div>CPU</div><div>{{ cpuMax }}</div>
          </th>
          <th
            class="text-left"
            style="min-width:110px"
          >
            RAM
          </th>
          <th
            class="text-left"
            style="min-width:110px"
          >
            <div>Encoder</div><div>uptime</div>
          </th>
    
          <th class="text-left">
            Enabled
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="config in configs"
          :key="config.uid"
        >
          <td>
            <v-tooltip
              text="Configure the Live Stream"
              location="bottom"
            >
              <template #activator="{ props }">
                <NuxtLink
                  v-bind="props"
                  class="text-primary text-decoration-none "
                  :to="'/ff/config?configUid=' + config.uid"
                >
                  {{ config.name }}
                </NuxtLink>
              </template>
            </v-tooltip>
          </td>
          <td>
            <v-tooltip
              text="Monitor the Live Stream"
              location="bottom"
            >
              <template #activator="{ props }">
                <v-btn
                  v-bind="props"
                  density="default"
                  variant="plain"
                  icon="mdi-play-circle"
                  :to="'/ff/monitor?&configUid=' + config.uid"
                />
              </template>
            </v-tooltip>  
          </td>
          <td>
            <ff-status-icon :config-uid="config.uid" />
          </td>
          <td>{{ config.acceleration }}</td>
          <td>
            {{ nbVideoRepresentations(config.uid ?? '') }}
          </td>
          <td>
            {{ speed(config.uid) }}
          </td>
          <td>
            {{ fps(config.uid) }}
          </td>
          <td>
            {{ getStatus(config.uid)?.dropFrames }} / {{ getStatus(config.uid)?.duplicateFrames }}
          </td>
          <td>
            {{ cpuUage(config.uid) }}
          </td>
          <td>
            {{ memoryUsage(config.uid) }}
          </td>
          <td>
            {{ broadcastDuration(config.uid) }}
          </td>
       
          <td>
            <ff-enable-stream :config-uid="config.uid" />
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>
<script setup lang="ts">
const _props = defineProps<{
    serverUid: string
    nbCpus: number
}>();

const allConfigs = useFFConfigs();
const status = useFFStatus();
const utils = useUtils();

function getStatus(uid: string) {
    return status.value.find((s) => s.configUid === uid);
  }

const configs = computed(() => {
    return allConfigs.value.filter((config) => config.serverUid === _props.serverUid).sort((a, b) => a.name.localeCompare(b.name));
  });

function getConfig(uid: string) {
    return configs.value.find((c) => c.uid === uid);
  }

function nbVideoRepresentations(uid: string) {
    const config = getConfig(uid);
    if (config == undefined) {
      return 0;
    }

    let ret = 0;
    for (let i = 0; i < config.videoAdaptationSet.length; i++) {
      ret += config.videoAdaptationSet[i].representations.length;
    }

    return ret;
  }

  const cpuMax = computed(() => {
    if (_props.nbCpus === 0) {
      return "";
    }

    return "(max " + _props.nbCpus * 100 + "%)";
  });

  function fps(uid: string) {
    const fps = getStatus(uid)?.fps;
    if (!fps) {
      return "N/A";
    }
  
    return fps + "Hz";
  }

  function speed(uid: string) {
    const speed = getStatus(uid)?.speed;
    if (!speed) {
      return "N/A";
    }
  
    return "x" + speed;
  }
  
  function cpuUage(uid: string) {
    const usage = getStatus(uid)?.cpuUsage;
    if (!usage) {
      return "N/A";
    }
  
    return (Number(usage) / 100).toFixed(2) + "%";
  }

  function memoryUsage(uid: string) {
    const usage = getStatus(uid)?.memoryUsage;
    if (!usage) {
      return "N/A";
    }
  
    return utils.formatBytes(Number(usage));
  }
  
  function broadcastDuration(uid: string) {
    const duration = getStatus(uid)?.outTimeMs;
    if (!duration) {
      return "N/A";
    }
    return formatDuration(BigInt(duration));
  }
  
  function formatDuration(milliseconds: BigInt) {
    const seconds = Math.floor((Number(milliseconds) / 1000) % 60);
    const minutes = Math.floor((Number(milliseconds) / (1000 * 60)) % 60);
    const hours = Math.floor((Number(milliseconds) / (1000 * 60 * 60)) % 24);
    const days = Math.floor(Number(milliseconds) / (1000 * 60 * 60 * 24));
    const parts = [];
    if (days > 0) {
      parts.push(`${days}d`);
    }
    if (hours > 0) {
      parts.push(`${hours}h`);
    }
    if (minutes > 0) {
      parts.push(`${minutes}m`);
    }
  
    parts.push(`${seconds}s`);
  
    return parts.join(" ");
  }

</script>