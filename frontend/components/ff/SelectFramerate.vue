<template>
  <v-combobox
    v-model="resolution"
    label="Frame Rate"
    :items="list"
    filter-mode="every"
    suffix="fps"
    variant="underlined"
    density="compact"
  />
</template>
  <script setup lang="ts">
  
  const num = defineModel<number>('num')
  const den = defineModel<number>('den')
  
  const resolution = computed({
    get() {
      return `${num.value}/${den.value}`;
    },
    set(value: any) {
      const v = value.value === undefined ? value : value.value;
      const [n, d] = v.split("/");
      num.value = Number(n);
      if (d == undefined) {
        den.value =1
      } else {
        den.value = Number(d);
      }
    },
  });
  
  const list = [
     { title: "24p", value: "24/1"},
     { title: "25p", value: "25/1"},
     { title: "30p", value: "30/1"},
     { title: "50p", value: "50/1"},
     { title: "60p", value: "60/1"},
  ]
  
  </script>