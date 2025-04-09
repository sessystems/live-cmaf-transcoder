<template>
  <v-combobox
    v-model="resolution"
    label="Resolution"
    :items="resolutionList"
    filter-mode="every"
    suffix="px"
    variant="underlined"
    density="compact"
  />
</template>
<script setup lang="ts">

const width = defineModel<number>('width')
const height = defineModel<number>('height')

const resolution = computed({
  get() {
    return `${width.value}x${height.value}`;
  },
  set(value: any) {
    const v = value.value === undefined ? value : value.value;
    const [w, h] = v.split("x");
    width.value = Number(w);
    height.value = Number(h);
  },
});

const resolutionList = [
   { title: "3840x2160 (UHD)", value: "3840x2160"},
   { title: "1920x1080 (FullHD)", value: "1920x1080"},
   { title: "1280x720 (HD)", value: "1280x720"},
   { title: "960x540 (SD)", value: "960x540"},
   { title: "725x576 (SD)", value: "720x576"},
   { title: "640x360 (SD)", value: "640x360"},
   { title: "Original (Input Stream)", value: "0x0"}
]

</script>