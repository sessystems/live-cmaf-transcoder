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
        v-if="hasUICapability"
        target="_blank"
        class="text-info"
        :to="serverURL"
      >
        {{ serverURL }}
      </NuxtLink>
      <div
        v-else
        class="text-grey"
        :to="serverURL"
      >
        {{ serverURL }}
      </div>
    </template>

    <v-card-text>
      <v-row>
        <v-col lg="1" style="min-width: 160px;">VERSION: v{{ server?.version }} </v-col>
        <v-col lg="1" style="min-width: 130px;">CPU: {{ (serverStatus?.cpuUsage ?? 0) / 100 }}% </v-col>
        <v-col lg="1" style="min-width: 110px;"> CORES: {{ serverStatus?.nbCpus ?? 0 }} </v-col>
        <v-col style="min-width: 120px;">
          RAM: {{ utils.formatBytes(Number(serverStatus?.memoryUsage ?? 0)) }} /
          {{ utils.formatBytes(Number(serverStatus?.totalMemory ?? 0)) }}
        </v-col>
      </v-row>
      <v-row v-if="!hasTranscodeCapability && hasUICapability">
        <v-col>      
          <NuxtLink
          v-if="hasUICapability"
          target="_blank"
          class="text-info"
          :to="serverURL"
        >
          Management UI
          </NuxtLink>
        </v-col>
      </v-row>
      <ff-overview
        v-if="hasTranscodeCapability"
        :server-uid="_props.serverUid"
        :nb-cpus="serverStatus?.nbCpus ?? 0"
      />
      
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ServerCapability } from "~/backend/models/ServerCapability";

const _props = defineProps<{
  serverUid: string;
}>();
const utils = useUtils();
const servers = useServers();
const api = useApi();
const allServerStatus = useServerStatus();

const hasTranscodeCapability = computed(() => {
  return (
    server.value?.capabilities.includes(ServerCapability.Transcode) === true
  );
});

const serverStatus = computed(() => {
  return allServerStatus.value.find(
    (status) => status.serverUid === _props.serverUid
  );
});

const hasUICapability = computed(() => {
  return (
    serverStatus.value?.currentState === "Running" &&
    server.value?.capabilities.includes(ServerCapability.UserInterface) === true
  );
});

const server = computed(() => {
  return servers.value.find((server) => server.uid === _props.serverUid);
});

const serverURL = computed(() => {
  return server.value?.baseUrl;
});

async function deleteServer() {
  await api.removeServer(_props.serverUid);
}
</script>
