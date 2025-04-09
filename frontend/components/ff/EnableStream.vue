<template>
  <v-switch
    v-model="enabled"
    label=""
    color="success"
    density="compact"
  />
</template>
<script setup lang="ts">
const _props = defineProps<{
  configUid: string;
}>();

const ffapi = useFFApi();
const config = ffapi.getConfig(_props.configUid)
const enabled = ref(config?.state == "Active");

watch(enabled, async (value: boolean) => {
    if (config == null || config == undefined) return;
    config.state = value ? "Active" : "Inactive";
    await ffapi.setState(config.serverUid, config.uid, config.state);
});
</script>
