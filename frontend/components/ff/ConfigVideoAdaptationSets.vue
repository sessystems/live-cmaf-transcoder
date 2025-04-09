<template>
  <v-card
    elevation="0"
  >
    <v-card-title>
      <v-tooltip
        text="Add a new video Adaptation Set"
        location="top"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            icon="mdi-plus"
            color="info"
            variant="flat"
            density="compact"
            @click.prevent="addAdaptationSet()"
          />
        </template>
      </v-tooltip>
      Video Adaptation Sets ({{ config.videoAdaptationSet.length }})
    </v-card-title>

    <ff-config-video-adaptation-set
      v-for="(video_adaptation_set, video_adaptation_set_index) in config.videoAdaptationSet"
      :key="video_adaptation_set_index"
      v-model="config.videoAdaptationSet[video_adaptation_set_index]"
      :title="`Video Adaptation Set ${video_adaptation_set_index}`"
      :config="config"
      @remove="removeAdaptationSet(video_adaptation_set_index)"
    />
  </v-card>
</template>
<script setup lang="ts">
import type { FFConfig } from '~/backend/models/FFConfig';
const config = defineModel<FFConfig>({ required: true})
const ffapi = useFFApi();

function removeAdaptationSet(adaptation_set_index: number) {
  config.value.videoAdaptationSet.splice(adaptation_set_index, 1);
}

async function addAdaptationSet() {
  const adaptation_set = await ffapi.createDefaultVideoAdaptationSet();
  config.value.videoAdaptationSet.push(adaptation_set);
}
</script>