<template>
  <v-card
    title="FFMPEG Command (BASH)"
  >
    <v-card-text>
      {{ ffcmd }}
    </v-card-text>
      
    <v-card-actions>
      <v-spacer />
      <v-tooltip
        text="Copy to clipboard"
        location="bottom"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            icon="mdi-content-copy"
            @click.prevent="utils.copyToClipboard(ffcmd)"
          />
        </template>
      </v-tooltip>
    </v-card-actions>
  </v-card>
</template>
<script setup lang="ts">
  const _props = defineProps<{
    serverUid: string
    configUid: string,
  }>();
  
  const ffapi = useFFApi();
  const utils = useUtils();
  const ffcmd = ref("");
  onMounted(async () => {
      ffcmd.value = await ffapi.getFFCmd(_props.serverUid, _props.configUid);
  });

</script>