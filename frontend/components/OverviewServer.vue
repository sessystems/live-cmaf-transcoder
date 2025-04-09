<template>
  <v-card :title="server?.name">
    <template #prepend>
      <server-status-icon :server-uid="server?.uid ?? ''" />
    </template>

    <template #append>
      <delete-popup
        v-if="serverStatus?.currentState !== 'Running'"
        tooltip="Delete this server"
        text="Are you sure you want to delete this server from the Database ?"
        title="Delete this server"
        variant="text"
        @remove="deleteServer"
      />
      <NuxtLink
        v-if="serverStatus?.currentState === 'Running'"
        target="_blank"
        class="text-info"
        :to="serverURL"
      >
        {{ serverURL }}
      </NuxtLink>
    </template>

    <v-card-text>
      <v-row>
        <v-col lg="1">
          VERSION: v{{ server?.version }}
        </v-col>
        <v-col lg="1">
          CPU: {{ (serverStatus?.cpuUsage ?? 0) / 100 }}% 
        </v-col>
        <v-col lg="1">
          CORES: {{ serverStatus?.nbCpus ?? 0 }}
        </v-col>
        <v-col>
          RAM: {{ utils.formatBytes(Number(serverStatus?.memoryUsage ?? 0)) }} / {{ utils.formatBytes(Number(serverStatus?.totalMemory ?? 0)) }}
        </v-col>
      </v-row>
      <ff-overview
        v-if="server?.capabilities.includes('Transcode')"
        :server-uid="_props.serverUid"
        :nb-cpus="serverStatus?.nbCpus ?? 0"
      />
    </v-card-text>
  </v-card>
</template>
  
<script setup lang="ts">
const _props = defineProps<{
    serverUid: string
}>();
  const utils = useUtils();
  const servers = useServers();
  const api = useApi();
  const allServerStatus = useServerStatus();
  
  const server = computed(() => {
    return servers.value.find((server) => server.uid === _props.serverUid);
  });

  const serverStatus = computed(() => {
    return allServerStatus.value.find((status) => status.serverUid === _props.serverUid);
  });

  const serverURL = computed(() => {
    return server.value?.baseUrl;
  });

  async function deleteServer() {
    await api.removeServer(_props.serverUid);
  }

</script>
