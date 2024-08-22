<script setup lang="ts">
import type { ElectionCreateScore } from '@/backend'
import { ref, type PropType } from 'vue'
import CreateCandidates from '@/components/CreateCandidates.vue'

const props = defineProps({
  modelValue: {
    type: Object as PropType<ElectionCreateScore>,
    required: true
  }
})

const emits = defineEmits<{
  (e: 'update:modelValue', value: ElectionCreateScore): void
}>()

const max_score = ref(props.modelValue.max_score)
</script>

<template>
  <div>
    <div>
      <label for="max_score">Max Score </label><br />
      <input
        type="number"
        id="max_score"
        v-model="max_score"
        @change="
          emits(`update:modelValue`, {
            ...props.modelValue,
            max_score: max_score
          })
        "
      /><br />
    </div>
    <br />
    <CreateCandidates v-model="modelValue.options" />
  </div>
</template>

<style scoped>
hr {
  margin-top: 3px;
  margin-bottom: 8px;
}
</style>
