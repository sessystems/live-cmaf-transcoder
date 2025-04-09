<template>
  <v-container fluid>
    <v-snackbar
      v-model="alert.visible.value"
      color="transparent"
      elevation="0"
      variant="text"
    >
      <v-alert
        :text="alert.content.value"
        :title="alert.title.value"
        :type="alert.type.value"
      />
    </v-snackbar>

    <v-form ref="form" @submit.prevent>
      <v-row justify="center">
        <v-col xl="7" lg="10">
          <ff-config-input-stream v-model="config" />

          <ff-config-video-adaptation-sets v-model="config" />

          <ff-config-audio-adaptation-sets v-model="config" />

          <ff-config-output-stream v-model="config" />

          <row-apply-config-btn
            :modified="modified"
            @apply="apply"
            @reset="reset"
            @remove="remove"
          />
        </v-col>
      </v-row>
    </v-form>
  </v-container>
</template>

<script setup lang="ts">
import type { FFConfig } from "~/backend/models/FFConfig";
const alert = useAlert();
const ffapi = useFFApi();
const configs = useFFConfigs();
const form: Ref<any> = ref(null);
if (configs.value.length == 0) {
  await ffapi.refresh_configs();
}
const route = useRoute();
const configUid = ref(route.query.configUid as string);
const configValue = JSON.stringify(ffapi.getConfig(configUid.value));
const origin = reactive({} as FFConfig);
const config = reactive({} as FFConfig);
Object.assign(config, JSON.parse(configValue));
Object.assign(origin, JSON.parse(configValue));

const title = useTitle();
const currentConfigUid = useCurrentConfigUid();

onMounted(() => {
  title.value = "Live Transcoder Configuration";
  currentConfigUid.value = configUid.value;
});

const modified = computed(() => {
  const configString = JSON.stringify(config);
  const originString = JSON.stringify(origin);
  return configString !== originString;
});

async function validate() {
  const { valid } = await form.value.validate();
  return valid;
}

async function remove() {
  await ffapi.remove(origin.serverUid, configUid.value);
  await navigateTo("/");
}

async function apply() {
  if (!(await validate())) {
    alert.setError("Error", "Please fix the errors before saving");
    return;
  }

  if (config.serverUid !== origin.serverUid) {
    await ffapi.remove(origin.serverUid, configUid.value);
  }

  await ffapi.update(config);
  alert.setSuccess("Success", "Settings have been saved successfully");
  reset();
}

function reset() {
  const configValue = JSON.stringify(ffapi.getConfig(configUid.value));
  Object.assign(config, JSON.parse(configValue));
  Object.assign(origin, JSON.parse(configValue));
}
</script>
