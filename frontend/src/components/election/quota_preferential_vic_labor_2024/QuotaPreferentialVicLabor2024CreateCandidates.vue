<script setup lang="ts">
import type { QuotaPreferentialVicLabor2024Candidate } from '@/backend'
import { onMounted, ref, watch } from 'vue'

const props = defineProps<{
  candidates: QuotaPreferentialVicLabor2024Candidate[]
}>()

const emits = defineEmits<{
  (e: 'updated', value: QuotaPreferentialVicLabor2024Candidate[]): void
}>()

const min_candidates = ref(2)
const max_candidates = ref(100)
const candidates = ref<QuotaPreferentialVicLabor2024Candidate[]>(props.candidates)

watch(candidates.value, () => {
  emits('updated', candidates.value)
})

onMounted(() => {
  emits('updated', candidates.value)
})

function remove_candidate(index: number) {
  candidates.value.splice(index, 1)
}

function add_candidate() {
  candidates.value.push({ name: '', is_female: true })
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
          <input type="text" v-model="candidates[i].name" />
          <label>Is Woman</label>
          <input type="checkbox" v-model="candidates[i].is_female" />
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
label {
  margin-left: 5px;
  margin-right: 3px;
}

hr {
  margin-top: 3px;
  margin-bottom: 8px;
}
</style>
