<script setup lang="ts">
import { ElectionType } from '@/backend'
import { ref, type PropType } from 'vue'

const props = defineProps({
  modelValue: {
    type: Object as PropType<ElectionType>,
    required: true
  },
  disabled: {
    type: Object as PropType<ElectionType[]>,
    default: () => {
      return []
    }
  }
})

const emits = defineEmits<{
  (e: 'update:modelValue', value: ElectionType): void
}>()

let election_types = ref<ElectionType[]>(Object.values(ElectionType))

let selected = ref<ElectionType>(props.modelValue)
</script>

<template>
  <div>
    <select v-model="selected" @change="() => emits(`update:modelValue`, selected)">
      <option
        v-for="election_type in election_types"
        :value="election_type"
        :key="election_type"
        :disabled="disabled.includes(election_type)"
      >
        {{ election_type }}
      </option>
    </select>
  </div>
</template>

<style scoped></style>
