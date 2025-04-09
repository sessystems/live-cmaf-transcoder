<template>
  <v-container fluid>
    <v-row>
      <v-col>
        <ff-video-player :config-uid="configUid" />
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <logs-view
          :server-uid="config?.serverUid ?? ''"
          :config-uid="configUid"
        />
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <ff-cmd
          :server-uid="config?.serverUid ?? ''"
          :config-uid="configUid"
        />
      </v-col>
    </v-row>
  </v-container>
</template>
<script setup lang="ts">

const title = useTitle();
title.value = "CMAF Transcoder Monitoring";

const ffapi = useFFApi();
const configs = useFFConfigs();
if (configs.value.length === 0) {
    await ffapi.refresh_configs();
}

const route = useRoute();
const configUid = ref(route.query.configUid as string);
const currentConfigUid = useCurrentConfigUid();
currentConfigUid.value = configUid.value;
const config = computed(() => {
    return configs.value.find((config) => config.uid === configUid.value);
});

</script>