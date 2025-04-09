<template>
  <v-dialog max-width="500">
    <template #activator="{ props: activatorProps }">
      <v-tooltip
        text="Create a new Live Stream Encoder"
        location="bottom"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="mergeProps(activatorProps, props)"
            color="info"
            text="New live stream encoder"
            variant="flat"
            prepend-icon="mdi-video-plus"
          />
        </template>
      </v-tooltip>
    </template>
  
    <template #default="{ isActive }">
      <v-card title="New Live Stream Encoder">
        <v-form
          ref="form"
          @submit.prevent="onCreate"
        >
          <v-card-text>
            Please enter the name of the Live Stream you want to create
            <v-text-field
              v-model="title"
              placeholder="Name of the Live Stream"
              :rules="nameRules"
              density="compact"
            />
          </v-card-text>
  
          <v-card-actions>
            <v-spacer />
  
            <v-btn
              text="Ok"
              color="success"
              type="submit"
            />
            <v-btn
              text="Cancel"
              color="error"
              @click.prevent="isActive.value = false"
            />
          </v-card-actions>
        </v-form>
      </v-card>
    </template>
  </v-dialog>
</template>
<script setup lang="ts">
import { mergeProps } from 'vue'
const _props = defineProps<{
  serverUid: string;
}>();
const title = ref<String>("");
const ffapi = useFFApi();
const form: Ref<any> = ref(null)

const nameRules = [
  (v: any) => !!v || 'Name is required',
  (v: string) => (v && v.length > 0)  || 'Name is required',
];

async function validate () {
  const { valid } = await form.value.validate();
  return valid;
}

async function onCreate() {

  if (await validate() == false) {
    return;
  }

  const output = "/live/" + title.value.toString()
      .replace(" ", "_")
      .replace("/", "_")
      .replace("\\", "_")
      .replace(":", "_")
      .replace("*", "_")
      .replace("?", "_")
      .replace("\"", "_")
      .replace("<", "_")
      .replace(">", "_")
      .replace("|", "_");

  const configUid = await ffapi.create(_props.serverUid, title.value.toString(), output);
  await navigateTo("/ff/config?configUid=" + configUid);
}
</script>