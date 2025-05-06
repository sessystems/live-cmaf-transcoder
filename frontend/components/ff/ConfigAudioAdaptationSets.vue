<template>
  <v-card elevation="0" class="my-2">
    <v-card-title> Audio Representations </v-card-title>
    <v-card-text>
      <v-row dense>
        <v-col cols="6" lg="3">
          <v-select
            v-model="config.audioAdaptationSet.encoder"
            :items="audioEncoderList"
            label="Audio Encoder"
            variant="underlined"
            density="compact"
            :error="isEncoderError"
            :error-messages="encoderErrorMessage"
          />
        </v-col>
        <v-col cols="6" lg="2">
          <v-select
            v-model="config.audioAdaptationSet.profile"
            :items="audioProfileFiles"
            label="Audio Profile"
            variant="underlined"
            density="compact"
            :error="isAudioProfileError"
            :error-messages="audioProfileErrorMessage"
          />
        </v-col>
        <v-col cols="6" lg="3">
          <ff-select-bitrate
            v-model="config.audioAdaptationSet.bitrate"
            :audio="true"
            label="Bitrate"
          />
        </v-col>
        <v-col cols="6" lg="2">
          <ff-select-sample-rate v-model="config.audioAdaptationSet.sampleRate" />
        </v-col>
        <v-col cols="6" lg="2">
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
import { Acceleration, AudioEncoder, Audioprofile } from "~/backend";
import type { FFConfig } from "~/backend/models/FFConfig";
const config = defineModel<FFConfig>({ required: true });

const audioEncoderList = computed(() => {
  if (config.value.acceleration === Acceleration.Software) {
    return [{ title: "AAC (FFMPEG)", value: "Aac" }];
  }

  return [
    { title: "AAC (Fraunhofer FDK)", value: AudioEncoder.FdkAac },
    { title: "AAC (FFMPEG)", value: AudioEncoder.Aac },
  ];
});

const audioProfileFiles = [
  { title: "LC", value: Audioprofile.Low },
  { title: "HE-AAC", value: Audioprofile.Heaac },
  { title: "HE-AAC v2", value: Audioprofile.Heaacv2 },
];

const isEncoderError = computed(() => {
  const contain = audioEncoderList.value.findIndex(
    (e) => e.value === config.value.audioAdaptationSet.encoder
  );

  return contain < 0;
});

const encoderErrorMessage = computed(() => {
  if (isEncoderError.value) {
    return "FDK AAC encoder is not available with software acceleration";
  }
  return undefined;
});

const isAudioProfileError = computed(() => {
  if (config.value.audioAdaptationSet.encoder === AudioEncoder.FdkAac) {
    return false;
  }

  if (
    config.value.audioAdaptationSet.profile === Audioprofile.Heaac ||
    config.value.audioAdaptationSet.profile === Audioprofile.Heaacv2
  ) {
    return true;
  }

  return false;
});

const audioProfileErrorMessage = computed(() => {
  if (isAudioProfileError.value) {
    return "HE-AAC is not supported with AAC (FFMPEG) encoder";
  }
  return undefined;
});

const audioRoleList = ["Main"];
</script>
