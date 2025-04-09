<template>
  <v-row
    dense
  >
    <v-col>
      <v-btn
        min-width="100px"
        class="ma-4"
        :disabled="!modified"
        color="success"
        prepend-icon="mdi-content-save"
        @click.prevent="apply"
      >
        Save
      </v-btn>
      <v-btn
        min-width="100px"
        :disabled="!modified"
        color="warning"
        class="ma-4"
        prepend-icon="mdi-reload"
        @click.prevent="reset"
      >
        Reset
      </v-btn>

      <delete-popup
        btn-class="ma-4"
        class="ma-4"
        @remove="remove"
      />
    </v-col>
    <v-spacer />
  </v-row>
</template>
<script setup lang="ts">
  defineProps<{
    modified: boolean,
  }>();
  const emit = defineEmits(['apply', 'reset', 'remove']);
  function remove() {
    emit('remove');
  }
  function apply() {
    emit('apply');
  }
  function reset() {
    emit('reset');
  }
</script>