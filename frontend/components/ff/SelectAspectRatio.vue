<template>
  <v-combobox
    v-model="aspectRatio"
    label="Aspect Ratio"
    :items="list"
    filter-mode="every"
    variant="underlined"
    density="compact"
  />
</template>
<script setup lang="ts">
const num = defineModel<number | undefined | null>("num");
const den = defineModel<number | undefined | null>("den");

const aspectRatio = computed({
  get() {
    if (!num.value || !den.value) {
      return "Default";
    }

    return `${num.value}/${den.value}`;
  },
  set(value: any) {
    const v = value.value === undefined ? value : value.value;
    if (v === "") {
      num.value = null;
      den.value = null;
      return;
    }

    const [n, d] = v.split("/");
    num.value = Number(n);
    if (d == undefined) {
      den.value = 1;
    } else {
      den.value = Number(d);
    }
  },
});

const list = [
  { title: "Default", value: "Default" },
  { title: "16/9", value: "16/9" },
  { title: "4/3", value: "4/3" },
];
</script>
