<script setup lang="ts">
import { ref, type PropType } from 'vue'

const props = defineProps({
  modelValue: {
    type: Object as PropType<string[]>,
    required: true
  },
  min_candidates: {
    type: Number,
    required: false,
    default: 2
  },
  max_candidates: {
    type: Number,
    required: false,
    default: 100
  }
})

const emits = defineEmits<{
  (e: 'update:modelValue', value: string[]): void
}>()

const candidates = ref(props.modelValue)

function remove_candidate(index: number) {
  candidates.value.splice(index, 1)
  emits('update:modelValue', candidates.value)
}

function add_candidate() {
  candidates.value.push('')
  emits('update:modelValue', candidates.value)
}
</script>

<template>
  <div>
    <table>
      <tr>
        <th>Number</th>
        <th>Candidate</th>
        <th>Remove</th>
      </tr>
      <tr v-for="(_, i) in candidates" :key="i">
        <td>{{ i + 1 }}</td>
        <td>
          <input
            type="text"
            v-model="candidates[i]"
            @change="emits(`update:modelValue`, candidates)"
          />
        </td>
        <td>
          <button v-if="candidates.length > min_candidates" @click="remove_candidate(i)">
            remove
          </button>
        </td>
      </tr>
    </table>
    <button @click="add_candidate" v-if="candidates.length < max_candidates">Add Another</button>
  </div>
</template>

<style scoped>
hr {
  margin-top: 3px;
  margin-bottom: 8px;
}
</style>
