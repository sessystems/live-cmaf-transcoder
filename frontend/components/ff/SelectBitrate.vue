<template>
  <v-combobox
    v-model="bitrateKb"
    :label="_props.label"
    :items="_props.audio ? bitrateAudio : bitrateVideo"
    filter-mode="every"
    suffix="kb/s"
    variant="underlined"
    density="compact"
  />
</template>
<script setup lang="ts">

const bitrate = defineModel<number>({ required: true})
const _props = defineProps<{ label: string, audio: boolean }>();

const bitrateKb = computed({
    get() {
        return '' + Number(bitrate.value) / 1000;
    },
    set(value: any) {
        if (value.value === undefined) {
            bitrate.value = Number(value) * 1000;
        } else {
            bitrate.value = Number(value.value) * 1000;
        }
    },
});

const bitrateVideo = [{ title: '16Mb/s', value: 16000 }, { title: '8Mb/s', value: 8000 }, { title: '6Mb/s', value: 6000 }, { title: '4.5Mb/s', value: 4500 }, { title: '3Mb/s', value: 3000 },
{ title: '2Mb/s', value: 2000 }, { title: '1Mb/s', value: 1000 }, { title: '730kb/s', value: 730 }, { title: '365kb/s', value: 365 }];

const bitrateAudio = [{ title: '192kb/s', value: 192 }, { title: '128kb/s', value: 128 }, { title: '96kb/s', value: 96 }, { title: '64kb/s', value: 64 }];

</script>