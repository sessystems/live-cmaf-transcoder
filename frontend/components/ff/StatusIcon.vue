<template>
  <v-tooltip
    :text="state"
    location="top"
  >
    <template #activator="{ props }">
      <v-icon
        v-bind="props"
        :icon="icon"
        size="x-large"
        :color="color"
      />
    </template>
  </v-tooltip>
</template>
<script setup lang="ts">
const _props = defineProps<{
  configUid: string;
}>();

const status = useFFStatus();
const state = computed(() => status.value.find((s) => s.configUid === _props.configUid)?.currentState ?? "Unknown");
const icon = computed(() => {
  switch (state.value) {
    case "Running":
      return "mdi-video-check";
    case "Waiting":
      return "mdi-video-minus";
    case "Stopped":
      return "mdi-video-off";
    case "Error":
      return "mdi-alert-box";
    default:
      return "mdi-circle-slice-8";
  }
});

const color = computed(() => {
  switch (state.value) {
    case "Running":
      return "green";
    case "Waiting":
      return "orange";
    case "Stopped":
      return "grey";
    case "Error":
      return "red";
    default:
      return "grey";
  }
});
</script>