<template>
  <v-combobox
    v-model="framerate"
    label="Frame Rate"
    v-model:menu="opened"
    :items="list"
    item-title="title"
    no-filter
    suffix="fps"
    variant="underlined"
    density="compact"
  >
  </v-combobox>
</template>
<script setup lang="ts">
const num = defineModel<number>("num");
const den = defineModel<number>("den");
const opened = ref(false);

const framerate = computed({
  get() {
    if (opened.value) {
      return `${num.value}/${den.value}`;
    }

    const value = `${num.value}/${den.value}`;
    const defaultValue = den.value == 1 ? `${num.value}` : `${num.value}/${den.value}`;
    return list.find((item) => item.value === value)?.title ?? defaultValue;
  },
  set(value: any) {
    const v = value.value === undefined ? value : value.value;
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
  { title: "24p", value: "24/1" },
  { title: "25p", value: "25/1" },
  { title: "30p", value: "30/1" },
  { title: "50p", value: "50/1" },
  { title: "60p", value: "60/1" },
];
</script>
