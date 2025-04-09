<template>
  <v-card elevation="0">
    <v-card-title> Input Stream </v-card-title>

    <v-card-text>
      <v-row dense>
        <v-col>
          <v-text-field
            v-model="config.name"
            label="Name of the Stream"
            variant="underlined"
            density="compact"
          />
        </v-col>

        <v-col>
          <v-text-field
            v-model="config.input.uri"
            label="URL (srt|udp|http://<ip>:<port>)"
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>
      <v-row dense>
        <v-col>
          <v-select
            v-model="config.input.mode"
            :items="srtModeList"
            label="Mode (SRT Only)"
            :disabled="config.input.uri.startsWith('srt://') === false"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col>
          <v-text-field
            v-model.number="config.input.connectTimeoutMs"
            type="number"
            label="Connection Timeout (SRT Only)"
            suffix="milliseconds"
            :disabled="config.input.uri.startsWith('srt://') === false"
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col>
          <v-switch
            v-model="config.deinterlace.enable"
            label="Deinterlace"
            color="info"
            density="compact"
          />
        </v-col>
        <v-col>
          <v-select
            v-model="gpu"
            :items="gpuList"
            :rules="[
              () => gpu != undefined || 'Please select the acceleration',
            ]"
            label="Acceleration"
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import type { FFConfig } from "~/backend/models/FFConfig";
import type { Gpu } from "~/backend/models/Gpu";
const config = defineModel<FFConfig>({ required: true });
const servers = useServers();
const server = computed(() =>
  servers.value.find((s) => s.uid === config.value.serverUid)
);
const srtModeList = ["Caller", "Listener", "Rendezvous"];
const gpu = computed({
  get() {
    return server.value?.gpus.find((gpu) => gpu.uid === config.value.gpuUid);
  },
  set(value: Gpu | undefined) {
    if (value == undefined) {
      return;
    }
    config.value.gpuUid = value.uid;
    config.value.acceleration = value.acceleration;
  },
});

const gpuList = computed(() => {
  return (
    server.value?.gpus.map((gpu) => ({
      title: "[" + gpu.index + "] " + gpu.name,
      value: gpu,
    })) ?? []
  );
});
</script>
