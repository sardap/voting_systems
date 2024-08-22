<script setup lang="ts">
import type { QuotaPreferentialVicLabor2024Candidate } from '@/backend'
import { ref, watch } from 'vue'
import QuotaPreferentialVicLabor2024CreateCandidates from '@/components/election/quota_preferential_vic_labor_2024/QuotaPreferentialVicLabor2024CreateCandidates.vue'

const props = defineProps<{
  elected_count: number
  candidates: QuotaPreferentialVicLabor2024Candidate[]
}>()

const emit = defineEmits<{
  (
    e: 'updated',
    value: { candidates: QuotaPreferentialVicLabor2024Candidate[]; elected_count: number }
  ): void
}>()

const elected_count = ref(props.elected_count)
const candidates = ref<QuotaPreferentialVicLabor2024Candidate[]>(props.candidates)

watch(elected_count, () => {
  console.log('Updated create')
  emit('updated', {
    elected_count: elected_count.value,
    candidates: candidates.value
  })
})

watch(candidates.value, () => {
  console.log('Updated create')
  emit('updated', {
    elected_count: elected_count.value,
    candidates: candidates.value
  })
})
</script>

<template>
  <div>
    <div>
      <label for="elected_count">Elected Count </label><br />
      <input type="number" id="elected_count" v-model="elected_count" /><br />
    </div>
    <br />
    <QuotaPreferentialVicLabor2024CreateCandidates
      :candidates="candidates"
      @updated="
        (updated) => {
          candidates = updated
          emit('updated', {
            elected_count: elected_count,
            candidates: candidates
          })
        }
      "
    />
  </div>
</template>

<style scoped></style>
