<script setup lang="ts">
import type { SntvCreateElection } from '@/backend'
import { ref, type PropType } from 'vue'
import CreateCandidates from '@/components/CreateCandidates.vue'

const props = defineProps({
  modelValue: {
    type: Object as PropType<SntvCreateElection>,
    required: true
  }
})

const emits = defineEmits<{
  (e: 'update:modelValue', value: SntvCreateElection): void
}>()

const elected_count = ref(props.modelValue.elected_count)
</script>

<template>
  <div>
    <div>
      <label for="elected_count">Elected Count </label><br />
      <input
        type="number"
        id="elected_count"
        v-model="elected_count"
        @change="
          emits(`update:modelValue`, {
            ...props.modelValue,
            elected_count: elected_count
          })
        "
      /><br />
    </div>
    <br />
    <CreateCandidates v-model="modelValue.options" />
  </div>
</template>

<style scoped></style>
