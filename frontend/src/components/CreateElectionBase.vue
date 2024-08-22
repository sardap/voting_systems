<script setup lang="ts">
import type { CreateElection } from '@/backend'
import { ref, type PropType } from 'vue'

const props = defineProps({
  modelValue: {
    type: Object as PropType<CreateElection>,
    required: true
  }
})

const emits = defineEmits<{
  (e: 'update:modelValue', value: CreateElection): void
}>()

const to_update = ref(props.modelValue)
</script>

<template>
  <div>
    <div>
      <label for="title">Title</label><br />
      <input
        type="text"
        id="title"
        v-model="to_update.title"
        @change="emits(`update:modelValue`, to_update)"
      /><br />
    </div>
    <div>
      <label for="require_token">Require Token </label>
      <input
        type="checkbox"
        id="require_token"
        v-model="to_update.require_token"
        @change="emits(`update:modelValue`, to_update)"
      /><br />
    </div>
  </div>
</template>

<style scoped>
hr {
  margin-top: 3px;
  margin-bottom: 8px;
}
</style>
