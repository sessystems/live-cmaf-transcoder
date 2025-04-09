<template>
  <v-dialog
    v-model="dialog"
    max-width="500px"
  >
    <template #activator="{ props: activatorProps }">
      <v-tooltip
        :text="_props.tooltip ?? 'Delete this Live Stream'"
        location="bottom"
      >
        <template #activator="{ props }">
          <v-btn
            v-bind="mergeProps(activatorProps, props)"
            :color="_props.color ?? 'error'"
            text="Delete"
            :variant="_props.variant ?? 'flat'"
            :class="_props.btnClass"
            min-width="100px"
            prepend-icon="mdi-delete"
          />
        </template>
      </v-tooltip>
    </template>
    
    <template #default="{ isActive }">
      <v-card
        :title="_props.title ?? 'Delete'"
      >
        <v-card-text>
          {{ _props.text ?? 'Are you sure you want to delete this Live Stream ?' }}
        </v-card-text>
    
        <v-card-actions>
          <v-spacer />
    
          <v-btn
            text="Yes"
            color="error"
            @click.prevent="emit('remove')"
          />
          <v-btn
            text="Cancel"
            @click.prevent="isActive.value = false"
          />
        </v-card-actions>
      </v-card>
    </template>
  </v-dialog>
</template>
<script setup lang="ts">
  import { mergeProps } from 'vue'
  const _props = defineProps<{
    text?: string,
    tooltip?: string
    title?: string
    btnClass?: string
    color?: string
    variant?: "text" | "flat" | "elevated" | "tonal" | "outlined" | "plain"
  }>();
  const dialog = defineModel<boolean>();
  const emit = defineEmits(['remove']);
</script>