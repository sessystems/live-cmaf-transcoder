<template>
  <v-card

    elevation="0"
  >
    <v-card-title>
      Audio Representations
    </v-card-title>
    <v-card-text>
      <v-row dense>
        <v-col
          cols="6"
          lg="3"
        >
          <v-select
            v-model="config.audioAdaptationSet.encoder"
            :items="audioEncoderList"
            label="Audio Encoder"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col
          cols="6"
          lg="2"
        >
          <v-select
            v-model="config.audioAdaptationSet.profile"
            :items="audioProfileFiles"
            label="Audio Profile"
            variant="underlined"
            density="compact"
          />
        </v-col>
        <v-col
          cols="6"
          lg="3"
        >
          <ff-select-bitrate
            v-model="config.audioAdaptationSet.bitrate"
            :audio="true"
            label="Bitrate"
          />
        </v-col>
        <v-col
          cols="6"
          lg="2"
        >
          <ff-select-sample-rate
            v-model="config.audioAdaptationSet.sampleRate"
          />
        </v-col>  
        <v-col
          cols="6"
          lg="2"
        >
          <v-select
            v-model="config.audioAdaptationSet.role"
            :items="audioRoleList"
            label="Role"
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
const config = defineModel<FFConfig>({ required: true});
const audioEncoderList = [{ title: "AAC (Fraunhofer FDK)", value: "FDKAac" }, { title: "AAC (FFMPEG)", value: "Aac" }];
const audioProfileFiles = [{ title: "LC", value: "Low" }, { title: "HE-AAC", value: "HEAAC" }, { title: "HE-AAC v2", value: "HEAACV2" }];
const audioRoleList = ["Main"];
</script>