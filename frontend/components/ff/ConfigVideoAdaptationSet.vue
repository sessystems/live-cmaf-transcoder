<template>
  <v-card class="ma-4">
    <v-card-title>
      <v-tooltip
        text="Remove video Adaptation Set"
        location="top"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            icon="mdi-delete"
            color="red"
            variant="text"
            density="compact"
            @click.prevent="emit('remove')"
          />
        </template>
      </v-tooltip>
    
      {{ _props.title }}
    </v-card-title>

    <v-card-text>
      <v-row dense>
        <v-col>
          <v-select
            v-model="video_adaptation_set.encoder"
            :items="videoEncoderList"
            label="Video Encoder"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col>
          <ff-select-framerate
            v-model:num="video_adaptation_set.framerateNum"
            v-model:den="video_adaptation_set.framerateDen"
          />
        </v-col>
        <v-col>
          <ff-select-aspect-ratio
            v-model:num="video_adaptation_set.aspectRatioNum"
            v-model:den="video_adaptation_set.aspectRatioDen"
          />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col>
          <ff-create-representation-popup
            v-model="video_adaptation_set"
          />
          Video Representations ({{ video_adaptation_set.representations.length }})
        </v-col>
      </v-row>

      <ff-config-video-representation
        v-for="(representation, index) in video_adaptation_set.representations"
        :key="index"
        v-model="video_adaptation_set.representations[index]"
        :title="`Representation ${index}`"
        :config="_props.config"
        @remove="removeRepresentation(index)"
      />
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import type { FFConfig } from '~/backend/models/FFConfig';
import type { VideoAdaptationSet } from '~/backend/models/VideoAdaptationSet';
const video_adaptation_set = defineModel<VideoAdaptationSet>({ required: true})
const _props = defineProps<{ title: string, config: FFConfig }>()
const emit = defineEmits(['remove'])

const videoEncoderList   = [
{ title: "H264", value: "H264" },
{ title: "HEVC", value: "HEVC" },
];

function removeRepresentation(index: number) {
    video_adaptation_set.value.representations.splice(index, 1);
}



</script>