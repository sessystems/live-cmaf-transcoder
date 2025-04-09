<template>
  <v-dialog
    v-model="dialog"
    max-width="500px"
  >
    <template #activator="{ props: activatorProps }">
      <v-tooltip
        text="Create a new Video Representation"
        location="bottom"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="mergeProps(activatorProps, props)"
            color="info"
            variant="flat"
            density="compact"
            icon="mdi-plus"
          />
        </template>
      </v-tooltip>
    </template>
      
    <template #default="{ isActive }">
      <v-card
        title="Select Video Representation Template"
      >
        <v-form
          ref="form"
          @submit.prevent="addRepresentation"
        >
          <v-card-text>
            <v-select
              v-model="representationTemplate"
              :items="representationTemplateList"
              density="compact"
            />
          </v-card-text>
      
          <v-card-actions>
            <v-spacer />
      
            <v-btn
              text="Create"
              color="info"
              type="submit"
            />
            <v-btn
              text="Cancel"
              @click.prevent="isActive.value = false"
            />
          </v-card-actions>
        </v-form>
      </v-card>
    </template>
  </v-dialog>
</template>
  <script setup lang="ts">
    import { mergeProps } from 'vue'
    import type { VideoAdaptationSet } from '~/backend/models/VideoAdaptationSet';
    const videoAdaptationSet = defineModel<VideoAdaptationSet>({ required: true})
    const dialog = ref(false);
    const ffapi = useFFApi();
    const form = ref(null);
  
    const representationTemplate = ref('4k');
    const representationTemplateList = [
      { title: '4k (3840x2160 at 16Mb/s)', value: '4k' },
      { title: 'FullHD (1920x1080 at 8Mb/s)', value: 'fullhd' },
      { title: 'HD (1280x720 at 4Mb/s)', value: 'hd' },
      { title: 'SD (720x576 at 2Mb/s)', value: 'sd' },
      { title: 'SD (640x480 at 1Mb/s)', value: 'sd2' },
    ];

    watch(() => dialog.value, (value) => {
      if (value) {
        if (videoAdaptationSet.value.representations.length > 0) {
          const last = videoAdaptationSet.value.representations[videoAdaptationSet.value.representations.length -1];
          console.log(last.width);
          if (last.width <= 720) {
            representationTemplate.value = "sd2";
          } else if (last.width <= 1280) {
            representationTemplate.value = "sd";
          } else if (last.width <= 1920) {
            representationTemplate.value = "hd";
          } else if (last.width <= 3840) {
            representationTemplate.value = "fullhd";
          }
        } else {
          representationTemplate.value = "4k";
        }
      }
    });

    async function addRepresentation() {
    const r = await ffapi.createDefaultVideoRepresentation();

    
    if (videoAdaptationSet.value.representations.length > 0) {
      r.preset = videoAdaptationSet.value.representations[videoAdaptationSet.value.representations.length - 1].preset;
      r.profile = videoAdaptationSet.value.representations[videoAdaptationSet.value.representations.length - 1].profile;
    }

    if (representationTemplate.value === "4k") {
      r.width = 3840;
      r.height = 2160;
      r.bitrate = 16000000;
      r.maxBitrate = 16000000;
    } else if (representationTemplate.value === "fullhd") {
      r.width = 1920;
      r.height = 1080;
      r.bitrate = 8000000;
      r.maxBitrate = 8000000;
    } else if (representationTemplate.value === "hd") {
      r.width = 1280;
      r.height = 720;
      r.bitrate = 4000000;
      r.maxBitrate = 4000000;
    } else if (representationTemplate.value === "sd") {
      r.width = 720;
      r.height = 576;
      r.bitrate = 2000000;
      r.maxBitrate = 2000000;
    } else if (representationTemplate.value === "sd2") {
      r.width = 640;
      r.height = 480;
      r.bitrate = 1000000;
      r.maxBitrate = 1000000;
    }

  videoAdaptationSet.value.representations.push(r);
  dialog.value = false;
}
  </script>