<template>
  <v-tooltip
    :text="tooltip"
    location="top"
  >
    <template #activator="{ props }">
      <v-icon
        v-bind="props"
        :icon="icon"
        size="large"
        :color="color"
      />
    </template>
  </v-tooltip>
</template>
  <script setup lang="ts">
  const _props = defineProps<{
    serverUid: string;
  }>();
  
  const status = useServerStatus();
  const state = computed(() => status.value.find((s) => s.serverUid === _props.serverUid)?.currentState ?? "Unknown");
  const icon = computed(() => {
    switch (state.value) {
      case "Running":
        return "mdi-check-network";
      default:
        return "mdi-network-off";
    }
  });
  
  const color = computed(() => {
    switch (state.value) {
      case "Running":
        return "green";
      default:
        return "red";
    }
  });

  const tooltip = computed(() => {
    switch (state.value) {
      case "Running":
        return "Connected";
      default:
        return "Disconnected";
    }
  });

  </script>