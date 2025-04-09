<template>
  <v-combobox
    v-model="samplerateKHz"
    label="Sample Rate"
    :items="samplerateList"
    filter-mode="every"
    suffix="Khz"
    variant="underlined"
    density="compact"
  />
</template>
  <script setup lang="ts">
  
  const samplerate = defineModel<number>({ required: true})
  const samplerateKHz = computed({
      get() {
          return '' + Number(samplerate.value) / 1000;
      },
      set(value: any) {
          if (value.value === undefined) {
            samplerate.value = Number(value) * 1000;
          } else {
            samplerate.value = Number(value.value) * 1000;
          }
      },
  });
  
  const samplerateList = [{ title: '96kHz', value: 96 }, { title: '48kHz', value: 48 }, { title: '44.1kHz', value: 44.1 }, { title: '22kHz', value: 22 }];
  
  </script>