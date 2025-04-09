<template>
  <v-card
    class="bg-surface-light mt-4"
  >
    <v-card-title>
      <v-tooltip
        text="Delete this video representation"
        location="top"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            color="red"
            icon="mdi-delete"
            variant="text"
            @click.prevent="emit('remove')"
          />
        </template>
      </v-tooltip> {{ _props.title }}
    </v-card-title>

    <v-card-text>
      <v-row
    
        dense
      >
        <v-col
          cols="6"
          lg="4"
        >
          <ff-select-resolution
            v-model:width="representation.width"
            v-model:height="representation.height"
          />
        </v-col>
        <v-col
          cols="6"
          lg="2"
        >
          <ff-select-bitrate
            v-model="representation.bitrate"
            :audio="false"
            label="Bitrate"
          />
        </v-col>
        <v-col
          cols="6"          
          lg="2"
        >
          <ff-select-bitrate
            v-model="representation.maxBitrate"
            :audio="false"
            :rules="[(v:any) => representation.bitrate <= representation.maxBitrate || 'Max Bitrate < Bitrate']"
            label="Max Bitrate"
          />
        </v-col>
        <v-col
          cols="6"
          lg="2"
        >
          <v-select
            v-model="representation.preset"
            :disabled="_props.config.acceleration !== 'Software'"
            :items="presetList"
            label="Preset"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col
          cols="6"
          lg="2"
        >
          <v-select
            v-model="representation.profile"
            :items="profileList"
            label="Profile"
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>


      <v-row
    
        dense
      >
        <v-col
          cols="6"
          lg="2"
        >
          <v-switch
            v-model="representation.text.enabled"
            density="compact"
            label="Text Insertion"
            color="info"
          />
        </v-col>
        <v-col
          v-if="representation.text.enabled"
          cols="6"
          lg="5"
        >
          <v-combobox
            v-model="representation.text.text"
            filter-mode="every"
            :items="textComplete(representation)"
            label="Text"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col
          v-if="representation.text.enabled"
          cols="6"
          lg="1"
        >
          <v-text-field
            v-model.number="representation.text.fontsize"
            type="number"
            label="Font Size"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col
          v-if="representation.text.enabled"
          cols="6"
          lg="2"
        >
          <v-text-field
            v-model="representation.text.color"
            label="Font Color"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col
          v-if="representation.text.enabled"
          cols="6"
          lg="2"
        >
          <v-select
            v-model="representation.text.position"
            :items="textPositionList"
            label="Text position"
            variant="underlined"
            density="compact"
          />
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import type { FFConfig } from '~/backend/models/FFConfig';
import type { VideoRepresentation } from '~/backend/models/VideoRepresentation';
const representation = defineModel<VideoRepresentation>({ required: true})
const _props = defineProps<{ title: string, config: FFConfig }>()
const emit = defineEmits(['remove'])
const utils = useUtils()

const textPositionList = [
  { title: "Top Left", value: "TopLeft" },
  { title: "Top Center", value: "TopCenter" },
  { title: "Top Right", value: "TopRight" },
  { title: "Middle Left", value: "MiddleLeft" },
  { title: "MiddleCenter", value: "MiddleCenter" },
  { title: "Middle Right", value: "MiddleRight" },
  { title: "Bottom Left", value: "BottomLeft" },
  { title: "Bottom Center", value: "BottomCenter" },
  { title: "Bottom Right", value: "BottomRight" },
];

function textComplete(representation: VideoRepresentation) {
  return [
  "" + representation.width + "x" + representation.height + " " + utils.formatBitrate(representation.bitrate),
    "%{localtime\\: %X}",
    "%{pts \\: hms}",
  ]
}

const presetList = [
  "Ultrafast",
  "Superfast",
  "Veryfast",
  "Faster",
  "Fast",
  "Medium",
  "Slow",
  "Slower",
  "Veryslow",
];

const profileList = [
  "Main",
  "High",
];

</script>